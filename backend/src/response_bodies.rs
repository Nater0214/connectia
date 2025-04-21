use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub username: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MeResponse {
    pub username: String,
    pub admin: bool,
}