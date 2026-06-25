use axum::{
    Router,
    routing::{get, post},
};

mod auth;
mod config;
mod db;
mod extractor;
mod handler;
mod models;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(handler::get_users).post(handler::create_user))
        .route(
            "/users/{id}",
            get(handler::get_user).delete(handler::delete_user),
        )
        .route("/login", post(handler::login));

    let listener = tokio::net::TcpListener::bind(config::SERVER_ADDRESS)
        .await
        .unwrap();

    db::setup().await;

    axum::serve(listener, app).await.unwrap();
}
