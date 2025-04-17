use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub username: String,
}
