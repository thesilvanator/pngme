mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    Ok(())
}
