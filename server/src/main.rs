// Global Imports
use axum::{
    Router,
    routing::{delete, get},
};
use serde::{Deserialize, Serialize};

// Configuration
const SERVER_ADDRESS: &'static str = "127.0.0.1:3000";
const SQLITE_DB_ADDRESS: &'static str = "sqlite:db.sqlite3";

// Data Structures
#[derive(Serialize, Debug, sqlx::FromRow)]
struct User {
    id: i64,
    username: String,
    password: String,
}

// Request Structures
#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
}
#[derive(Serialize)]
struct CreateUserResponse {
    id: i64,
}

// Database Handler
mod db {

    use crate::{SQLITE_DB_ADDRESS, User};

    use sqlx::SqlitePool;

    pub async fn setup() {
        let pool = get_pool().await;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id       INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create users table");
    }
    async fn get_pool() -> SqlitePool {
        let pool = SqlitePool::connect(SQLITE_DB_ADDRESS).await.unwrap();
        return pool;
    }

    pub async fn get_users() -> Vec<User> {
        let pool = get_pool().await;
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&pool)
            .await
            .expect("Could not Fetch Users from Database!");
        users
    }

    pub async fn create_user(username: &str, password: &str) -> i64 {
        let pool = get_pool().await;
        let result = sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
            .bind(username)
            .bind(password)
            .execute(&pool)
            .await
            .expect("Failed to Create User in Databse!");
        let user_id: i64 = result.last_insert_rowid();
        user_id
    }

    pub async fn delete_user(id: i64) {
        let pool = get_pool().await;
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .expect("Failed to Delete User!");
    }
}

// Request Handler
mod handler {
    use crate::{CreateUserRequest, CreateUserResponse, User, db};
    use axum::{Json, extract::Path};

    pub async fn get_users() -> Json<Vec<User>> {
        let users = db::get_users().await;
        Json(users)
    }
    pub async fn create_user(Json(body): Json<CreateUserRequest>) -> Json<CreateUserResponse> {
        let id: i64 = db::create_user(&body.username, &body.password).await;
        let response = CreateUserResponse { id: id };
        Json(response)
    }
    pub async fn delete_user(Path(id): Path<i64>) {
        db::delete_user(id).await;
    }
}

// Entry Point
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(handler::get_users).post(handler::create_user))
        .route("/users/{id}", delete(handler::delete_user));
    let listener = tokio::net::TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    db::setup().await;

    axum::serve(listener, app).await.unwrap();
}
