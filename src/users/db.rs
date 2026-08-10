// use crate::model::{CreateUser, UpdateUser, User, UserFullResponse};
use axum::extract::Json;
use sqlx::Result;

use crate::users::model::{CreateUser, UpdateUser, User, UserFullResponse};

pub async fn create_db_user(pool: &sqlx::PgPool, payload: CreateUser) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(&payload.name)
        .bind(&payload.email)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_db_user_byid(pool: &sqlx::PgPool, id: i32) -> Result<Json<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(Json(user))
}

pub async fn get_db_users(pool: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn update_db_user(
    pool: &sqlx::PgPool,
    id: i32,
    user: &UpdateUser,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET name = $1, email = $2 WHERE id = $3")
        .bind(&user.name)
        .bind(&user.email)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_db_user(pool: &sqlx::PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn find_user_by_email(
    pool: &sqlx::PgPool,
    email: String,
) -> Result<UserFullResponse, sqlx::Error> {
    let user = sqlx::query_as::<_, UserFullResponse>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}
