use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::{
    model::DefaultResponse,
    users::{
        db::{self, DbPool, get_connection},
        model::{UpdateUser, UserModified},
    },
};

pub async fn get_users(State(pool): State<DbPool>) -> Result<Json<Vec<UserModified>>, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result = db::list_users(&mut conn, 1, 10).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(result))
}

#[axum::debug_handler]
pub async fn get_user_by_id(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserModified>, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result = db::get_user_by_id(&mut conn, user_id).map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(result))
}

#[derive(Serialize)]
pub struct Count {
    count: i64,
}

pub async fn get_users_count(
    State(pool): State<DbPool>,
) -> Result<Json<Count>, Json<DefaultResponse>> {
    let mut conn = pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    let result = db::count_users(&mut conn);
    match result {
        Ok(count) => Ok(Json(Count { count })),
        Err(_) => Err(Json(DefaultResponse {
            success: false,
            message: "erro",
        })),
    }
}

pub async fn update_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(changes): Json<UpdateUser>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    let r = db::update_user(&mut conn, user_id, changes);
    match r {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = get_connection(&pool).unwrap();
    let r = db::delete_user(&mut conn, user_id);
    match r {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
