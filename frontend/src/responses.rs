use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub username: String,
}
