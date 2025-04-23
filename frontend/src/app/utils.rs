use gloo_net::http::Request;

use crate::net::responses;

use super::state;

/// An error in getting the current logged in user
#[derive(Debug)]
pub(super) enum GetCurrentUserError {
    GlooNetError(gloo_net::Error),
    ServerError(String),
    OtherError(String),
}

impl From<gloo_net::Error> for GetCurrentUserError {
    fn from(error: gloo_net::Error) -> Self {
        Self::GlooNetError(error)
    }
}

impl std::fmt::Display for GetCurrentUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlooNetError(error) => write!(f, "gloo-net error: {}", error),
            Self::ServerError(error) => write!(f, "Server error: {}", error),
            Self::OtherError(error) => write!(f, "Other error: {}", error),
        }
    }
}

impl std::error::Error for GetCurrentUserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::GlooNetError(err) => Some(err),
            Self::ServerError(_) => None,
            Self::OtherError(_) => None,
        }
    }
}

impl PartialEq for GetCurrentUserError {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

/// Get the current logged in user from the backend
pub(super) async fn get_current_user() -> Result<Option<state::User>, GetCurrentUserError> {
    // Construct the request
    let request = Request::get("/backend/current-user");

    // Get a response
    let response = request.send().await?;

    // Match the response status and return the appropriate result
    match response.status() {
        200 => {
            let response: responses::CurrentUserResponse = response.json().await?;
            Ok(Some(state::User {
                username: response.username,
                admin: response.admin,
            }))
        }
        401 => Ok(None),
        500 => Err(GetCurrentUserError::ServerError(response.text().await?)),
        code => Err(GetCurrentUserError::OtherError(format!(
            "Unexpected status code: {}",
            code
        ))),
    }
}
