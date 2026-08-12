use std::env;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use dotenv::dotenv;

use crate::{
    auth::jwt::JwtManager,
    model::DefaultResponse,
    users::{
        db::{create_db_user, delete_db_user, get_db_user_byid, get_db_users, update_db_user},
        model::{CreateUser, UpdateUser, User},
    },
};

pub async fn get_users(
    State(pool): State<sqlx::PgPool>,
    headers: HeaderMap,
) -> Result<Json<Vec<User>>, (StatusCode, Json<DefaultResponse>)> {
    let auth = headers.get("Authorization");
    if let Some(token) = auth {
        dotenv().ok();

        let secret_key = env::var("SECRET_KEY").unwrap();
        let mng = JwtManager::new(secret_key);
        let n = token.to_str().unwrap();
        let is_auth = mng.verify(n);
        match is_auth {
            Ok(r) => {
                println!("USER_AUTHORIZED: {}", r.email);

                get_db_users(&pool).await.map(Json).map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(DefaultResponse {
                            success: false,
                            message: "UZURLI SERVERDA MUAMMO BOP QOLDI",
                        }),
                    )
                })
            }
            Err(_) => {
                println!("WRONG TOKEN");
                Err((
                    StatusCode::UNAUTHORIZED,
                    Json(DefaultResponse {
                        success: false,
                        message: "BRAT TOKENIZ ESKI, YOKI XATO TOKEN, BITTA LOGIN QB KELIN",
                    }),
                ))
            }
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(DefaultResponse {
                success: false,
                message: "HEADERGA TOKEN QOYIB JONATIN BRAT",
            }),
        ))
    }
}

pub async fn create_user(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<DefaultResponse>), (StatusCode, Json<DefaultResponse>)> {
    create_db_user(
        &pool,
        CreateUser {
            name: payload.name,
            email: payload.email,
        },
    )
    .await
    .map(|_| {
        (
            StatusCode::CREATED,
            Json(DefaultResponse {
                success: true,
                message: "User created successfully",
            }),
        )
    })
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DefaultResponse {
                success: false,
                message: "ERROR",
            }),
        )
    })
}

pub async fn update_user(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<StatusCode, (StatusCode, String)> {
    update_db_user(&pool, id, &payload)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn delete_user(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<DefaultResponse>), (StatusCode, Json<DefaultResponse>)> {
    delete_db_user(&pool, id)
        .await
        .map(|_| {
            (
                StatusCode::OK,
                Json(DefaultResponse {
                    success: true,
                    message: "User deleted successfully",
                }),
            )
        })
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(DefaultResponse {
                    success: false,
                    message: "User does not exist",
                }),
            )
        })
}

#[axum::debug_handler]
pub async fn get_user_by_id(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, Json<DefaultResponse>)> {
    get_db_user_byid(&pool, id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(DefaultResponse {
                success: false,
                message: "User does not exist",
            }),
        )
    })
}
