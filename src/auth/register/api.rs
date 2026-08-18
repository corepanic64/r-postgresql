use axum::{Json, extract::State, http::StatusCode};

use crate::{
    auth::{password_manager::PasswordManager, register::db::register_user},
    model::DefaultResponse,
    users::{
        db::{DbPool, get_user_by_email},
        model::NewUser,
    },
};

pub async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<NewUser>,
) -> Result<StatusCode, (StatusCode, Json<DefaultResponse>)> {
    let hashed_password = PasswordManager::hash_password(payload.password.clone());
    let mut conn = pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DefaultResponse {
                success: false,
                message: "serverda xatolik",
            }),
        )
    })?;
    let user = get_user_by_email(&mut conn, payload.email.clone());
    println!("heel: {:?}", &payload);
    match user {
        Ok(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(DefaultResponse {
                success: false,
                message: "bu user mavjud !!!",
            }),
        )),
        Err(_) => register_user(
            &mut conn,
            NewUser {
                email: payload.email,
                username: payload.username,
                password: hashed_password.unwrap(),
            },
        )
        // .await
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
