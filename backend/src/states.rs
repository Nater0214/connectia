use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RootState {
    pub static_dir: PathBuf,
    pub reqwest_client: reqwest::Client,
}

impl RootState {
    pub fn new(static_dir: impl AsRef<Path>) -> RootState {
        RootState {
            static_dir: static_dir.as_ref().to_owned(),
            reqwest_client: reqwest::Client::new(),
        }
    }
}
