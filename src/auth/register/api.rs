use axum::{Json, extract::State, http::StatusCode};

use crate::{
    auth::{
        password_manager::PasswordManager,
        register::{db::register_db_user, model::Register},
    },
    model::DefaultResponse,
    users::db::find_user_by_email,
};

pub async fn register(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<Register>,
) -> Result<StatusCode, (StatusCode, Json<DefaultResponse>)> {
    let hashed_password = PasswordManager::hash_password(payload.password);
    let user = find_user_by_email(&pool, &payload.email).await;

    match user {
        Ok(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(DefaultResponse {
                success: false,
                message: "bu user mavjud !!!",
            }),
        )),
        Err(_) => register_db_user(
            &pool,
            Register {
                name: payload.name,
                email: payload.email,
                password: hashed_password.unwrap(),
            },
        )
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DefaultResponse {
                    success: false,
                    message: "serverda xatolik",
                }),
            )
        }),
    }
}
