use std::sync::{Arc, atomic::Ordering};

use axum::{
    body::Body,
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    http::{StatusCode, header},
    response::Response,
};
use futures::StreamExt;
use tokio::sync::RwLock;

use crate::{
    jwt::JwtData, packets::{ClientIn, ParsedMessage, encode_vote, parse, send}, state::{AppState, RoomState},
};

static CLIENT_HTML: &[u8] = include_bytes!("../../frontend/dist/client/client.html.gz");

pub async fn client_html() -> Response {
    Response::builder()
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .header(header::CONTENT_ENCODING, "gzip")
        .body(Body::from(CLIENT_HTML))
        .unwrap()
}

pub async fn client_ws(
    ws: WebSocketUpgrade,
    app_state: State<AppState>,
    jwt_data: JwtData,
) -> Result<Response, StatusCode> {
    let rooms = app_state.rooms.read().await;
    let room_state_lock = match rooms.get(&jwt_data.room_id) {
        Some(r) => r.clone(),
        None => return Err(StatusCode::NOT_FOUND),
    };

    drop(rooms);

    #[cfg(debug_assertions)]
    println!("WebSocket connection established for {}", jwt_data.id);

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, jwt_data.id, room_state_lock)))
}

async fn handle_socket(
    socket: WebSocket,
    client_id: usize,
    room_state_lock: Arc<RwLock<RoomState>>,
) {
    let (mut sender, mut receiver) = socket.split();

    let mut room_state = room_state_lock.write().await;
    
    // send initial vote only if not answered
    if let Some(current_vote) = &room_state.room.current_vote {
        let current_vote_id = room_state.current_vote_id();
        let client_last_vote_id = room_state.clients[client_id].last_vote_id.load(Ordering::Relaxed);

        if current_vote_id > client_last_vote_id {
            let _ = send(&mut sender, encode_vote(current_vote)).await;
        }
    }

    // register sender
    room_state.clients[client_id].sender = Some(sender);
    drop(room_state);

    while let Some(ress) = receiver.next().await {
        // read message
        let message = match ress {
            Ok(m) => m,
            Err(e) => {
                #[cfg(debug_assertions)]
                println!("error receiving ws message : {}", e);
                break;
            }
        };

        // parse message
        let answer_in = match parse::<ClientIn>(message) {
            ParsedMessage::Data(a) => a,
            ParsedMessage::Ping => continue,
            ParsedMessage::ParseError => {
                #[cfg(debug_assertions)]
                println!("format error expected AnswerIn");
                continue;
            }
        };

        #[cfg(debug_assertions)]
        println!("CLIENT: receiving new answer");

        // register vote
        let room_state = room_state_lock.read().await;

        // compare last client vote to current vote
        let current_vote_id = room_state.current_vote_id();
        let client_state = &room_state.clients[client_id];

        // set new client_last_vote_id and get the old value with atomic unicity
        let client_last_vote_id = client_state
            .last_vote_id
            .swap(current_vote_id, Ordering::Relaxed);

        // ignore if the vote has already been answered by client
        if client_last_vote_id >= current_vote_id {
            continue;
        }

        room_state
            .room
            .vote_for(answer_in.vote_id, answer_in.answer_id);
    }

    #[cfg(debug_assertions)]
    println!("client disconnected");

    // removing sender
    let mut room_state = room_state_lock.write().await;
    room_state.clients[client_id].sender = None;
}
