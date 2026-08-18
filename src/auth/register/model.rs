#[derive(Debug, serde::Deserialize)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}
