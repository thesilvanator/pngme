mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

use std::fs;

use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    match cli.command {
        args::Commands::Encode {
            path,
            chunk_type,
            message,
            output,
        } => {
            let contents = fs::read(&path)?;
            let mut png = png::Png::try_from(contents.as_slice())?;

            let ct = chunk_type::ChunkType::try_from(
                <[u8; 4]>::try_from(chunk_type.as_bytes())
                    .expect("error converting ct bytes to u8 array"),
            )?;

            let c = chunk::Chunk::new(ct, message.bytes().collect());

            png.append_chunk(c);

            match output {
                Some(o) => {
                    fs::write(o, png.as_bytes().as_slice())?;
                },
                None => {
                    fs::write(&path, png.as_bytes().as_slice())?;
                }
            }

            
        },
        args::Commands::Decode { path, chunk_type } => {
            let contents = fs::read(path)?;
            let png = png::Png::try_from(contents.as_slice())?;

            let chunk = png.chunk_by_type(&chunk_type);
            match chunk {
                Some(c) => println!("{c}"),
                None => println!("No Chunk found with type: {chunk_type}"),
            }

            
        },
        args::Commands::Remove { path, chunk_type } => {
            let contents = fs::read(&path)?;
            let mut png = png::Png::try_from(contents.as_slice())?;

            match png.remove_chunk(&chunk_type) {
                Ok(c) => println!("{c} removed"),
                Err(e) => println!("unable to remove chunk: {e}"),
            }
            
            fs::write(&path, png.as_bytes().as_slice())?;
        },
        args::Commands::Print { path } => {
            let contents = fs::read(path)?;
            let png = png::Png::try_from(contents.as_slice())?;

            println!("{png}");
        },
    }

    Ok(())
}
