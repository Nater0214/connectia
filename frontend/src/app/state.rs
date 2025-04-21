use yewdux::Store;

#[derive(Debug, Clone, Eq)]
pub(super) struct User {
    pub username: String,
    pub admin: bool,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

#[derive(Debug, Clone, PartialEq, Store)]
pub(super) struct State {
    pub current_user: Option<User>,
}

impl Default for State {
    fn default() -> Self {
        Self { current_user: None }
    }
}