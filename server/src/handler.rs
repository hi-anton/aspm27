use axum::{Json, extract::Path, http::StatusCode};

use crate::auth;
use crate::db;
use crate::models::{CreateUserResponse, LoginResponse, SendUserRequest, User};

pub async fn get_users() -> Json<Vec<User>> {
    let users = db::get_users().await;
    Json(users)
}

pub async fn create_user(Json(body): Json<SendUserRequest>) -> Json<CreateUserResponse> {
    let id: i64 = db::create_user(&body.username, &body.password).await;
    Json(CreateUserResponse { id })
}

pub async fn delete_user(Path(id): Path<i64>) {
    db::delete_user(id).await;
}

pub async fn get_user(Path(id): Path<i64>) -> Json<User> {
    let user: User = db::get_user_by_id(id).await;
    Json(user)
}

pub async fn login(Json(body): Json<SendUserRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let user: User = db::get_user_by_username(&body.username).await;
    let result: bool = auth::verify_password(&body.password, &user.password_hash);
    if result {
        Ok(Json(LoginResponse {
            token_string: auth::create_token(user.id),
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn get_user_chats() {}
pub async fn create_chat() {}
pub async fn get_chat() {}
