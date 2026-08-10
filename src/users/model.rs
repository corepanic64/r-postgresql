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

#[derive(sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserFullResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}
