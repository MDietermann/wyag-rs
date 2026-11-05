use std::path::PathBuf;

use clap::{Parser, Subcommand};
use wyag::repo::Repository;

#[derive(Parser)]
#[command(version, about = "Write yourself a git - in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create an empty repository
    Init {
        /// Where to create the repo
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { path } => {
            Repository::init(&path)?;
        }
    }

    Ok(())
}
