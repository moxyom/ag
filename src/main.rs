mod jwt;
mod routes;
mod state;
mod packets;
mod utils;

use axum::{
    Router,
    routing::{any, get, post},
};

use crate::{
    routes::{
        admin::{admin_html, admin_ws},
        client::{client_html, client_ws},
        me::me,
        room::{room_create, room_join},
    },
    state::AppState,
};

#[tokio::main]
async fn main() {
    // read .env
    dotenvy::dotenv().ok();
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // create app state
    let app_state = AppState::new(jwt_secret);

    let app = Router::new()
        .route("/", get(client_html))
        .route("/admin", get(admin_html))
        .route("/me", get(me))
        .route("/rooms/create", post(room_create))
        .route("/rooms/{room_id}/join", post(room_join))
        .route("/ws", any(client_ws))
        .route("/admin-ws", any(admin_ws))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
