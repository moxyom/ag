use std::sync::Arc;

use axum::{
    body::Body,
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    http::{StatusCode, header},
    response::Response,
};
use futures::StreamExt;
use tokio::sync::RwLock;

use crate::{
    jwt::JwtData,
    packets::{ClientIn, ParsedMessage, parse},
    state::{AppState, RoomState},
    utils::get_time_stamp,
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

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, room_state_lock)))
}

async fn handle_socket(socket: WebSocket, room_state_lock: Arc<RwLock<RoomState>>) {
    #[cfg(debug_assertions)]
    println!("CLIENT ws connection establised");

    let mut last_action_time_stamp = get_time_stamp();

    let (sender, mut receiver) = socket.split();

    // register sender
    let mut room_state = (*room_state_lock).write().await;
    let client_id = room_state.next_client_id.increment();
    room_state.client_senders.insert(client_id, sender);
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

        // don't count vote if connection happend after vote post
        if last_action_time_stamp > room_state.vote_post_time_stamp {
            continue;
        }

        room_state
            .room
            .vote_for(answer_in.vote_id, answer_in.answer_id);

        // update last action
        last_action_time_stamp = get_time_stamp();
    }

    #[cfg(debug_assertions)]
    println!("client disconnected");

    // removing sender
    let mut room_state = (*room_state_lock).write().await;
    room_state.client_senders.remove(&client_id);
}
