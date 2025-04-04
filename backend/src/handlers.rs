use axum::{
    extract::State,
    http,
    response::{Html, IntoResponse},
};
use tokio::{fs, io};

use crate::states::RootState;

/// A response error
#[derive(Debug)]
pub enum ErrorResponse {
    /// An IO error
    IoError(io::Error),
}

impl From<io::Error> for ErrorResponse {
    fn from(err: io::Error) -> Self {
        ErrorResponse::IoError(err)
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self)).into_response()
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorResponse::IoError(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl std::error::Error for ErrorResponse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorResponse::IoError(err) => Some(err),
        }
    }
}

pub async fn get_index(State(state): State<RootState>) -> Result<impl IntoResponse, ErrorResponse> {
    fs::read_to_string(state.static_dir.join("frontend/index.html"))
        .await
        .map(|content| Html(content).into_response())
        .map_err(|err| ErrorResponse::IoError(err))
}
