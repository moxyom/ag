mod utils;
mod extractor;

pub use utils::{sign, verify};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtData {
    #[serde(rename = "roomId")]
    pub room_id: usize,
    pub admin: bool,
    pub exp: usize,
}
