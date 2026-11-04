use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Write yourself a git - in Rust")]
struct Cli {}

fn main() -> anyhow::Result<()> {
    Cli::parse();
    Ok(())
}
