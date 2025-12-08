use clap::Parser;

mod pack;
mod parse;
mod unpack;
mod view;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    Pack(Pack),
    View(View),
    Unpack(Unpack),
}

#[derive(Parser, Debug)]
struct Pack {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
}

#[derive(Parser, Debug)]
struct View {
    #[arg(short, long)]
    pub path: String,
}

#[derive(Parser, Debug)]
struct Unpack {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
}

fn main() {
    let args = Args::parse();
    let _ = match args.command {
        Commands::Pack(args) => crate::pack::pack(args),
        Commands::View(args) => crate::view::view(args),
        Commands::Unpack(args) => crate::unpack::unpack(args),
    };
}
