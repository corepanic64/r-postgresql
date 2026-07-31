use axum::extract::Path;
use sqlx::Result;

#[derive(sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, serde::Deserialize, sqlx::FromRow)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
}

pub async fn create_user(pool: &sqlx::PgPool, payload: CreateUser) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(&payload.name)
        .bind(&payload.email)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user(pool: &sqlx::PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(())
}

pub async fn get_users(pool: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn update_user(
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
