use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use cookie::time::Duration;
use rand::RngExt;
use tokio::sync::RwLock;

use crate::{
    jwt,
    state::{AppState, RoomState},
};

pub async fn room_create(app_state: State<AppState>) -> impl IntoResponse {
    let mut rooms = app_state.rooms.write().await;

    // generate a room_id
    let mut rng = rand::rng();
    let mut room_id: usize;
    loop {
        room_id = rng.random_range(100_000usize..999_999usize);
        if !rooms.contains_key(&room_id) {
            break;
        }
    }

    // create the room_state
    rooms.insert(room_id, Arc::new(RwLock::new(RoomState::new())));
    drop(rooms);

    #[cfg(debug_assertions)]
    println!("ADMIN: new room created : {}", room_id);

    // create cookie
    let token = jwt::sign(room_id, true, &app_state.jwt_secret);
    let cookie = Cookie::build(("auth", token))
        .http_only(false)
        .same_site(SameSite::Lax)
        .max_age(Duration::days(1))
        .path("/")
        .build();

    (
        CookieJar::new().add(cookie),
        Json(serde_json::json!({ "roomId": room_id })),
    )
}

pub async fn room_join(
    app_state: State<AppState>,
    Path(room_id): Path<usize>,
) -> Result<CookieJar, StatusCode> {
    let rooms = (*app_state.rooms).read().await;
    let room_state_lock = match rooms.get(&room_id) {
        Some(r) => r,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // check if the room has been set unjoinable
    let room_state = room_state_lock.read().await;
    if !room_state.joinable {
        return Err(StatusCode::NOT_FOUND);
    }

    drop(room_state);

    #[cfg(debug_assertions)]
    println!("CLIENT: client joined : {}", room_id);

    // create cookie
    let token = jwt::sign(room_id, false, &app_state.jwt_secret);
    let cookie = Cookie::build(("auth", token))
        .http_only(false)
        .same_site(SameSite::Lax)
        .max_age(Duration::days(1))
        .path("/")
        .build();

    Ok(CookieJar::new().add(cookie))
}
