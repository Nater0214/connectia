use std::path::PathBuf;

use sea_orm::DatabaseConnection;

#[derive(Debug, Clone, Default)]
pub struct RootState {
    pub static_dir: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct BackendState {
    pub db_connection: DatabaseConnection,
}
