use axum::{Json, extract::State, http::StatusCode};

use crate::auth::{
    password_manager::PasswordManager,
    register::{db::register_db_user, model::Register},
};

pub async fn register(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<Register>,
) -> Result<StatusCode, (StatusCode, String)> {
    let hashed_password = PasswordManager::hash_password(payload.password);
    register_db_user(
        &pool,
        Register {
            name: payload.name,
            email: payload.email,
            password: hashed_password.unwrap(),
        },
    )
    .await
    .map(|_| StatusCode::CREATED)
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
