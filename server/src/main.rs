use axum::{
    Router,
    routing::{get, post},
};

use tower_http::cors::CorsLayer;

mod auth;
mod config;
mod db;
mod extractor;
mod handler;
mod models;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/chats",
            get(handler::get_user_chats).post(handler::create_chat),
        )
        .route("/chats/{id}", get(handler::get_chat))
        .route("/users", get(handler::get_users).post(handler::create_user))
        .route(
            "/users/{id}",
            get(handler::get_user).delete(handler::delete_user),
        )
        .route("/login", post(handler::login))
        // Erlaubt deinem Browser-Frontend Zugriffe
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(config::SERVER_ADDRESS)
        .await
        .unwrap();

    println!("Server läuft auf {}", config::SERVER_ADDRESS);

    db::setup().await;

    axum::serve(listener, app).await.unwrap();
}
