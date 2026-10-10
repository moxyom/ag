use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::jwt::JwtData;

pub fn sign(room_id: usize, admin: bool, secret: &Box<str>) -> String {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before Unix epoch")
        .as_secs() as usize
        + 24 * 60 * 60;

    jsonwebtoken::encode(
        &Header::default(),
        &JwtData { room_id, admin, exp },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("unable to encode token")
}

pub fn verify(token: &str, secret: &Box<str>) -> JwtData {
    let decoded = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .expect("unable to verify token");

    decoded.claims
}
