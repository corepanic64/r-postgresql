use axum::{Json, extract::State, http::StatusCode};

use crate::{
    auth::{
        login::model::{Login, LoginResponse},
        password_manager::PasswordManager,
    },
    users::db::find_user_by_email,
};

use crate::model::ErrorResponse;

#[axum::debug_handler]
pub async fn login(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<Login>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user_email = payload.email;
    let user_full = find_user_by_email(&pool, user_email).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "USER TOPILMADIUUUU".to_string(),
            }),
        )
    })?;
    let valid = PasswordManager::verify_password(&payload.password, &user_full.password);
    if !valid.unwrap() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "EE bratishka parol xatoku".to_string(),
            }),
        ));
    }

    Ok(Json(LoginResponse {
        token: "bu_sening_access_tokening buni hechkimga berma".to_string(),
    }))
}
