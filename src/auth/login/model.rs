#[derive(Debug, serde::Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
