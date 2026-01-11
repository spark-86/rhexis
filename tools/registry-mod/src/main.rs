use clap::Parser;

pub mod add;
pub mod generate;
pub mod remove;
pub mod update;
pub mod view;

pub use add::add;
pub use generate::generate;
pub use remove::remove;
pub use update::update;
pub use view::view;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
pub enum Commands {
    Add(Add),
    Update(Update),
    Remove(Remove),
    View(View),
    Generate(Generate),
}

#[derive(Parser, Debug)]
pub struct Generate {
    #[arg(short, long)]
    entry: String,
    #[arg(short, long)]
    data_type: String,
    #[arg(short, long)]
    value: Option<String>,
}

#[derive(Parser, Debug)]
pub struct Add {
    #[arg(short, long, required_unless_present = "new")]
    input: Option<String>,
    #[arg(short, long)]
    new: bool,
    #[arg(short, long)]
    entry: String,
    #[arg(short, long)]
    data_type: String,
    #[arg(short, long)]
    value: String,
    #[arg(short, long)]
    output: String,
}

#[derive(Parser, Debug)]
pub struct Update {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    entry: String,
    #[arg(short, long)]
    data_type: String,
    #[arg(short, long)]
    value: String,
    #[arg(short, long)]
    output: String,
}

#[derive(Parser, Debug)]
pub struct Remove {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    entry: String,
    #[arg(short, long)]
    output: String,
}

#[derive(Parser, Debug)]
pub struct View {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Cli::parse();
    let results = match args.command {
        Some(Commands::Add(args)) => add(args),
        Some(Commands::Update(args)) => update(args),
        Some(Commands::Remove(args)) => remove(args),
        Some(Commands::View(args)) => view(args),
        Some(Commands::Generate(args)) => generate(args),
        None => return,
    };
    println!("{:?}", results);
}
