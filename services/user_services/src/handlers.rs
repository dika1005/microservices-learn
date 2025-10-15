// services/user_services/src/handlers.rs
use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use sqlx::MySqlPool;
use crate::db;
use shared_types::{User, NewUser};

// Handler untuk mendapatkan semua user.
pub async fn list_users(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    match db::get_all_users(&pool).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Handler untuk membuat user baru.
pub async fn create_new_user(
    State(pool): State<MySqlPool>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    match db::create_user(&pool, payload).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}