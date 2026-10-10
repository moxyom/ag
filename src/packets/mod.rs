mod r#in;
mod out;

use futures::{SinkExt, StreamExt, stream};
pub use r#in::*;
pub use out::*;

use crate::state::RoomState;

impl RoomState {
    pub async fn send_to_clients(&mut self, payload: serde_json::Value) {
        stream::iter(self.client_senders.iter_mut())
            .for_each_concurrent(32, |(client_id, sender)| async {
                let ress = send(sender, payload.clone()).await;

                #[cfg(debug_assertions)]
                if ress.is_err() {
                    println!("Error sending vote to client {}", *client_id)
                }
            })
            .await;
    }

    pub async fn close_client_connections(&mut self) {
        stream::iter(self.client_senders.iter_mut())
            .for_each_concurrent(32, |(_, sender)| async {
                let _ = sender.close().await;
            })
            .await;
    }
}
