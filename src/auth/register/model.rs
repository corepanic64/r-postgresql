#[derive(Debug, serde::Deserialize, sqlx::FromRow)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}
