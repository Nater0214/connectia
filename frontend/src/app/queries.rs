use serde::{Deserialize, Serialize};

use super::Route;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) struct LoginQuery {
    #[serde(default)]
    pub next: Option<Route>,
}
