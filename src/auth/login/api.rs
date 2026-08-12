use std::env;

use axum::{Json, extract::State, http::StatusCode};
use dotenv::dotenv;

use crate::{
    auth::{
        jwt::JwtManager,
        login::model::{Login, LoginResponse},
        password_manager::PasswordManager,
    },
    model::DefaultResponse,
    users::db::find_user_by_email,
};

#[axum::debug_handler]
pub async fn login(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<Login>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<DefaultResponse>)> {
    let user_full = find_user_by_email(&pool, &payload.email)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(DefaultResponse {
                    success: false,
                    message: "User not found",
                }),
            )
        })?;
    let valid = PasswordManager::verify_password(&payload.password, &user_full.password);
    if !valid.unwrap() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(DefaultResponse {
                success: false,
                message: "You are not authorized",
            }),
        ));
    }
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").unwrap();
    let secret = JwtManager::new(secret_key);
    let token = secret.generate_token(user_full.id, payload.email).unwrap();
    Ok(Json(LoginResponse { token }))
}
