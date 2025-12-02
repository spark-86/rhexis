use clap::Parser;

mod pack;
mod unpack;
mod view;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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
    pub plugin_type: String,
    #[arg(short, long)]
    pub descriptor_path: String,
    #[arg(short, long)]
    pub code_path: String,
    #[arg(short, long)]
    pub output_path: String,
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
    let args = Cli::parse();
    if let Some(command) = args.command {
        match command {
            Commands::Pack(args) => crate::pack::pack(args),
            Commands::View(args) => crate::view::view(args),
            Commands::Unpack(args) => crate::unpack::unpack(args),
        }
    }
}
