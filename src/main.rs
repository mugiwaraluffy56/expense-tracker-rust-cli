use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser)]
#[command(version,
    about = "A rust based expense tracker CLI.",
    author = "Puneeth Aditya",
    propagate_version = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add(AddArgs),
}

#[derive(Debug)]
#[derive(Clone, ValueEnum)]
enum Category {
    Food,
    Shopping,
    Subscriptions,
}

#[derive(Args)]
struct AddArgs {
    category: Category,
    amount: f64,

    #[arg(long, short)]
    m: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Add(args) => {
            println!("Category: {:?}", args.category);
            println!("Amount: {}", args.amount);
            println!("Note: {:?}", args.m)
        }
    }
}