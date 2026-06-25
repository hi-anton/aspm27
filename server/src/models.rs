use serde::{Deserialize, Serialize};

// Data Structures
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

// Request Structures
#[derive(Deserialize)]
pub struct SendUserRequest {
    pub username: String,
    pub password: String,
}

// Response Structures
#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i64,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token_string: String,
}
