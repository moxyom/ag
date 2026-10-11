mod utils;
mod extractor;

use std::time::{SystemTime, UNIX_EPOCH};

pub use utils::{sign, verify};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtData {
    pub id: usize,
    #[serde(rename = "roomId")]
    pub room_id: usize,
    pub exp: usize,
}

impl JwtData {
    pub fn new(room_id: usize, client_id: Option<usize>) -> Self {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is before Unix epoch")
            .as_secs() as usize
            + 8 * 60 * 60;

        Self {
            id: client_id.unwrap_or(usize::MAX),
            room_id,
            exp,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.id == usize::MAX
    }
}
