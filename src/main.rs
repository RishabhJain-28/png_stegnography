use args::{Cli, Commands};
use clap::Parser;
use commands::{decode, encode, get_chunk_types, remove};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode(args) => {
            encode(args)?;
            println!("file encoded")
        }
        Commands::Decode(args) => {
            println!("{}", decode(args)?);
        }
        Commands::Remove(args) => {
            remove(args)?;
            println!("Chunk removed")
        }
        Commands::Print(args) => {
            let chgunk_types = get_chunk_types(args)?;
            for c in chgunk_types {
                println!("{}", c)
            }
        }
    }

    Ok(())
}
