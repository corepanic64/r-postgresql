use axum::{Json, extract::State, http::StatusCode, http::header::HeaderMap};

use crate::users::{
    db::{self, DbPool},
    model::User,
};

pub async fn get_users(State(pool): State<DbPool>) -> Result<Json<Vec<User>>, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result = db::list_users(&mut conn, 1, 10).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(result))
}
