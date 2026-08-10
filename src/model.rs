#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefaultResponse {
    pub success: bool,
    pub message: &'static str,
}
