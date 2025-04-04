use std::path::PathBuf;

use clap::Parser;

/// The arguments to this program
#[derive(Debug, Parser)]
#[command()]
pub struct ProgramArgs {
    /// The port to serve on
    #[arg(short, long)]
    pub port: Option<u16>,

    /// The directory to serve static files from
    #[arg(short, long)]
    pub static_dir: Option<PathBuf>,

    /// The logging verbosity
    #[arg(short, long)]
    pub verbosity: Option<String>,
}
