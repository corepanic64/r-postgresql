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
    users::db::{DbPool, get_user_by_email},
};

pub async fn login(
    State(pool): State<DbPool>,
    Json(payload): Json<Login>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<DefaultResponse>)> {
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
    match user {
        Ok(r) => {
            println!("user: {:?}", r);
            let valid = PasswordManager::verify_password(&payload.password, &r.password);
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
            let token = secret.generate_token(r.id, payload.email).unwrap();
            Ok(Json(LoginResponse { token }))
        }
        Err(e) => {
            println!("err: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DefaultResponse {
                    success: false,
                    message: "fuck you",
                }),
            ))
        }
    }
}
