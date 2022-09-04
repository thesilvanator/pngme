use clap::Parser;
use clap::AppSettings;
use clap::Subcommand;

use std::path::PathBuf;

#[derive(Parser)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]

#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode a message into a png chunk
    Encode {
        /// Path to png to encode into
        #[clap(value_parser)]
        path: PathBuf,
        /// Chunk type to be encoded
        #[clap(value_parser)]
        chunk_type: String,
        /// Message to be encoded
        #[clap(value_parser)]
        message: String,
        /// Location to output png
        #[clap(short, long, value_parser)]
        output: Option<PathBuf>,
    },
    /// Decode a message from a png chunk
    Decode {
        /// Path to png to decode from
        #[clap(value_parser)]
        path: PathBuf,
        /// Chunk type to be decoded
        #[clap(value_parser)]
        chunk_type: String,
    },
    /// Remove a png chunk
    Remove {
        /// Path to png to remove chunk from
        #[clap(value_parser)]
        path: PathBuf,
        /// Chunk type to be removed
        #[clap(value_parser)]
        chunk_type: String,
    },
    /// Print the chunks in a png
    Print {
        /// Path to png to print chunks
        #[clap(value_parser)]
        path: PathBuf,
    },
}
