use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};

use crate::db_requests::{
    create_db_user, delete_db_user, get_db_user_byid, get_db_users, update_db_user,
};
use crate::model::{CreateUser, UpdateUser, User};

pub async fn get_users(
    State(pool): State<sqlx::PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    get_db_users(&pool)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn create_user(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, (StatusCode, String)> {
    create_db_user(
        &pool,
        CreateUser {
            name: payload.name,
            email: payload.email,
        },
    )
    .await
    .map(|_| StatusCode::CREATED)
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
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
) -> Result<StatusCode, (StatusCode, String)> {
    delete_db_user(&pool, id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))
}

pub async fn get_user_by_id(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    get_db_user_byid(&pool, id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))
}
