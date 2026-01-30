use clap::Parser;

use crate::pack::pack_cache;

pub mod json;
pub mod pack;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    Pack(Pack),
}

#[derive(Parser, Debug)]
pub struct Pack {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
}

fn main() {
    let args = Cli::parse();
    let result = match args.command {
        Commands::Pack(args) => pack_cache(args),
    };
    if result.is_err() {
        eprintln!("{}", result.err().unwrap());
    }
}
