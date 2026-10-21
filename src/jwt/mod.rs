mod extractor;
mod utils;

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
pub use utils::{sign, verify};

#[derive(Serialize, Deserialize)]
pub struct JwtData {
    #[serde(
        serialize_with = "serialize_id",
        deserialize_with = "deserialize_id"
    )]
    pub id: usize,
    #[serde(rename = "roomId")]
    pub room_id: usize,
    pub exp: usize,
}

fn serialize_id<S>(id: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if *id == usize::MAX {
        serializer.serialize_i64(-1)
    } else {
        serializer.serialize_u64(*id as u64)
    }
}

fn deserialize_id<'a, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'a>,
{
    let id = i128::deserialize(deserializer)?;

    if id == -1 {
        Ok(usize::MAX)
    } else {
        usize::try_from(id).map_err(serde::de::Error::custom)
    }
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
