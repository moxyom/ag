use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::jwt::JwtData;

pub fn sign(jwt_data: &JwtData, secret: &Box<str>) -> String {
    jsonwebtoken::encode(
        &Header::default(),
        jwt_data,
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
