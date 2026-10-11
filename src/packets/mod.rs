mod r#in;
mod out;

use axum::extract::ws::{Message, WebSocket};
use futures::{
    StreamExt,
    stream::{self, SplitSink},
};
pub use r#in::*;
pub use out::*;

use crate::state::RoomState;

impl RoomState {
    pub async fn for_each_client_sender<F>(&mut self, f: F)
    where
        F: for<'a> AsyncFn(usize, &'a mut SplitSink<WebSocket, Message>),
    {
        stream::iter(self.clients.iter_mut().enumerate())
            .for_each_concurrent(32, async |(client_id, client_state)| {
                if let Some(sender) = &mut client_state.sender {
                    f(client_id, sender).await;
                }
            })
            .await;
    }
}
