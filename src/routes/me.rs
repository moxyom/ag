use axum::{Json, response::IntoResponse};
use serde_json::json;

use crate::jwt::JwtData;

pub async fn me(jwt_data: JwtData) -> impl IntoResponse {
    Json(json!(jwt_data))
}
