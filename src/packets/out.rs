use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, stream::SplitSink};
use serde_json::json;

use crate::state::{Room, Vote};

pub fn encode_vote(vote: &Vote) -> serde_json::Value {
    json!({
        "title": vote.title,
        "id": vote.id,
        "answers": vote.answers.iter().map(|a| &a.message).collect::<Vec<&Box<str>>>(),
    })
}

pub fn encode_stats(room: &Room) -> serde_json::Value {
    json!(room.get_current_stats())
}

pub fn encode_terminate() -> serde_json::Value {
    json!({ "terminate": true })
}

pub fn encode_incription_satus(open: bool) -> serde_json::Value {
    json!({ "open": open })
}

pub async fn send(
    sender: &mut SplitSink<WebSocket, Message>,
    value: serde_json::Value,
) -> Result<(), axum::Error> {
    sender.send(Message::Text(value.to_string().into())).await
}
