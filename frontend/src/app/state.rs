use yewdux::Store;

#[derive(Debug, Clone, Eq)]
pub struct User {
    pub username: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

#[derive(Debug, Clone, PartialEq, Store)]
pub struct State {
    pub current_user: Option<User>,
}

impl Default for State {
    fn default() -> Self {
        Self { current_user: None }
    }
}