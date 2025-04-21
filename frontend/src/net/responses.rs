use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CurrentUserResponse {
    pub username: String,
    pub admin: bool,
}