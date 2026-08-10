#[derive(Debug, serde::Deserialize, sqlx::FromRow)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
