use std::path::PathBuf;

use clap::Parser;

/// The arguments to this program
#[derive(Debug, Parser)]
#[command()]
pub struct ProgramArgs {
    /// The port to serve on
    #[arg(long)]
    pub port: Option<u16>,

    /// The directory to serve static files from
    #[arg(long)]
    pub static_dir: Option<PathBuf>,

    /// The url to the database
    #[arg(long)]
    pub database_url: Option<String>,

    /// Create a super user with a given name and password separated by a colon
    #[arg(long)]
    pub create_super_user: Option<String>,

    /// The logging verbosity
    #[arg(short, long)]
    pub verbosity: Option<String>,
}
