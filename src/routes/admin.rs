use std::{sync::Arc, time::Duration};

use axum::{
    body::Body,
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    http::{StatusCode, header},
    response::Response,
};
use futures::{SinkExt, StreamExt};
use tokio::{select, sync::RwLock, time::sleep};

use crate::{
    jwt::JwtData,
    packets::{
        AdminIn, ParsedMessage, encode_incription_satus, encode_stats, encode_terminate,
        encode_vote, parse, send,
    },
    state::{Answer, AppState, RoomState, Vote},
};

static ADMIN_HTML: &[u8] = include_bytes!("../../frontend/dist/admin/admin.html.gz");

pub async fn admin_html() -> Response {
    Response::builder()
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .header(header::CONTENT_ENCODING, "gzip")
        .body(Body::from(ADMIN_HTML))
        .unwrap()
}

pub async fn admin_ws(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
    jwt_data: JwtData,
) -> Result<Response, StatusCode> {
    if !jwt_data.is_admin() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let rooms = app_state.rooms.read().await;
    let room_state_lock = match rooms.get(&jwt_data.room_id) {
        Some(r) => r.clone(),
        None => return Err(StatusCode::NOT_FOUND),
    };

    drop(rooms);

    Ok(ws.on_upgrade(move |socket| {
        handle_socket(socket, app_state, jwt_data.room_id, room_state_lock)
    }))
}

async fn handle_socket(
    socket: WebSocket,
    app_state: AppState,
    room_id: usize,
    room_state_lock: Arc<RwLock<RoomState>>,
) {
    #[cfg(debug_assertions)]
    println!("ADMIN ws connection establised");

    let (mut sender, mut receiver) = socket.split();

    // send initial vote
    let room_state = room_state_lock.read().await;
    if let Some(current_vote) = &room_state.room.current_vote {
        send(&mut sender, encode_vote(current_vote)).await;
    }
    send(&mut sender, encode_stats(&room_state)).await;
    send(&mut sender, encode_incription_satus(room_state.joinable)).await;
    drop(room_state);

    loop {
        select! {
            _ = sleep(Duration::from_millis(500)) => {
                let room_state = room_state_lock.read().await;
                let ress = send(&mut sender, encode_stats(&room_state)).await;
                if ress.is_err() {
                    #[cfg(debug_assertions)]
                    println!("ERROR: connot send stats");
                    break
                }
            }
            next = receiver.next() => {
                let Some(ress) = next else {
                    #[cfg(debug_assertions)]
                    println!("ERROR: no more messages to be received");
                    break
                };

                // read message
                let message = match ress {
                    Ok(m) => m,
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        println!("error receiving admin ws message : {}", e);
                        break;
                    }
                };

                // parse action
                let action = match parse::<AdminIn>(message) {
                    ParsedMessage::Data(a) => a,
                    ParsedMessage::Ping => continue,
                    ParsedMessage::ParseError => {
                        #[cfg(debug_assertions)]
                        println!("format error expected AdminIn");
                        continue;
                    }
                };

                let mut room_state = room_state_lock.write().await;
                match action {
                    AdminIn::SetVoteInscription { open } => {
                        room_state.joinable = open;
                        continue
                    },
                    AdminIn::Terminate { terminate } => {
                        if !terminate { continue }

                        // remove hashMap entry
                        let mut rooms = app_state.rooms.write().await;
                        rooms.remove(&room_id);

                        // send terminate to all
                        room_state.for_each_client_sender(async |_, sender| {
                            send(sender, encode_terminate()).await;
                            sender.close().await;
                        }).await;
                        break
                    }
                    AdminIn::NewVoteIn { title, answers } => {
                        let vote_id = room_state.room.set_current_vote(
                            title.clone(),
                            answers.clone().into_iter().map(Answer::new).collect(),
                        );

                        let encoded_vote = encode_vote(&Vote {
                            title,
                            answers: answers.into_iter().map(Answer::new).collect(),
                            id: vote_id,
                        });

                        room_state.for_each_client_sender(async move |_, sender| {
                            send(sender, encoded_vote.clone()).await;
                        }).await;
                    },
                };
            }
        }
    }

    #[cfg(debug_assertions)]
    println!("admin disconnected");
}
