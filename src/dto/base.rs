use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BaseApiResponse {
    pub success: bool,
    pub message: Option<String>,
}