use clap::Parser;

use crate::pack::pack_cache;
use crate::view::view_cache;

pub mod json;
pub mod pack;
pub mod view;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    Pack(Pack),
    View(View),
}

#[derive(Parser, Debug)]
pub struct Pack {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
}

#[derive(Parser, Debug)]
pub struct View {
    #[arg(short, long)]
    pub path: String,
}

fn main() {
    let args = Cli::parse();
    let result = match args.command {
        Commands::Pack(args) => pack_cache(args),
        Commands::View(args) => view_cache(args),
    };
    if result.is_err() {
        eprintln!("{}", result.err().unwrap());
    }
}
