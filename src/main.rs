use chrono::{NaiveDate, Local};
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
    Init(CreateArgs),
    Add(AddArgs),
}

#[derive(Debug, Clone, ValueEnum)]
enum Category {
    Food,
    Shopping,
    Subscriptions,
}

#[derive(Args)]
struct AddArgs {
    category: Category,
    amount: f64,
    date: Option<NaiveDate>,

    #[arg(long, short)]
    note: Option<String>,
}

#[derive(Args)]
struct CreateArgs {
    db_name: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Add(args) => {
            println!("Category: {:?}", args.category);
            println!("Amount: {}", args.amount);
            println!("Note: {:?}", args.note);
            let date = args.date.unwrap_or(
                Local::now().date_naive()
            );
            println!("Date: {}", date)
        },

        Command::Init(args) => {
            println!("Created a new database named {}.", args.db_name)
        }
    }
}