use axum::{Json, extract::State, http::StatusCode};

use crate::{
    auth::{
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

    Ok(Json(LoginResponse {
        token: "bu_sening_access_tokening buni hechkimga berma".to_string(),
    }))
}
