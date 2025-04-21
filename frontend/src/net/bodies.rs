use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LoginBody {
    pub username: String,
    pub password: String,
}
