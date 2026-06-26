use axum::routing::get;
use sqlx::{SqlitePool, pool};

use crate::auth::hash_password;
use crate::config::SQLITE_DB_ADDRESS;
use crate::models::User;

pub async fn setup() {
    let pool = get_pool().await;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            username      TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
    //sqlx::query(
    //    "CREATE TABLE IF NOT EXISTS chats (
    //        id            INTEGER PRIMARY KEY AUTOINCREMENT,
    //        chat_name     TEXT NOT NULL,
    //    )",
    //)
    //.execute(&pool)
    //.await
    //.unwrap();
    //sqlx::query(
    //    "CREATE TABLE IF NOT EXISTS chat_members (
    //        chat_id INTEGER NOT NULL,
    //        user_id INTEGER NOT NULL,
    //
    //        PRIMARY KEY (chat_id, user_id),
    //
    //        FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE,
    //        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
    //    )",
    //)
    //.execute(&pool)
    //.await
    //.unwrap();
}

async fn get_pool() -> SqlitePool {
    SqlitePool::connect(SQLITE_DB_ADDRESS).await.unwrap()
}

pub async fn get_users() -> Vec<User> {
    let pool = get_pool().await;
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("Could not fetch users from database!")
}

pub async fn create_user(username: &str, password: &str) -> i64 {
    let pool = get_pool().await;
    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(hash_password(password))
        .execute(&pool)
        .await
        .expect("Failed to create user in database!");
    result.last_insert_rowid()
}

pub async fn delete_user(id: i64) {
    let pool = get_pool().await;
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .expect("Failed to delete user!");
}

pub async fn get_user_by_id(id: i64) -> User {
    let pool = get_pool().await;
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("Failed to get user!")
}

pub async fn get_user_by_username(username: &str) -> User {
    let pool = get_pool().await;
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(&pool)
        .await
        .expect("Failed to get user!")
}

//pub async fn create_chat(chat_name: &str) {
//    let pool = get_pool().await;
//    sqlx::query("INSERT INTO chats (chat_name) VALUES (?)")
//        .bind(chat_name)
//        .execute(&pool)
//        .await
//        .unwrap();
//}

//pub async fn delete_chat(chat_id: &str) {
//    let pool = get_pool().await;
//    sqlx::query("DELETE FROM chats WHERE chat_id = ?")
//        .bind(chat_id)
//        .execute(&pool)
//        .await
//        .unwrap();
//}

//pub async fn chat_add_user(chat_id: i64, user_id: &str) {
//    let pool = get_pool().await;
//    sqlx::query("INSERT INTO chat_members (chat_id, user_id) VALUES (?, ?)")
//        .bind(chat_id)
//        .bind(user_id)
//        .execute(&pool)
//        .await
//        .unwrap();
//}

//pub async fn chat_delete_user(chat_id: i64, user_id: i64) {
//    let pool = get_pool().await;
//    sqlx::query("DELETE * FROM chat_members WHERE chat_id = ? AND user_id = ?")
//        .bind(chat_id)
//        .bind(user_id)
//        .execute(&pool)
//        .await
//        .unwrap();
//}
