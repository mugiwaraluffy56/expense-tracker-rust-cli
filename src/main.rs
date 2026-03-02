use chrono::{Datelike, NaiveDate};
use clap::{Args, Parser, Subcommand};

mod database;
mod date_utils;
mod expense;
mod summary;

#[derive(Parser)]
#[command(
    name = "ept",
    version,
    about = "A minimal expense tracker CLI",
    author = "Puneeth Aditya",
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize the database
    Init,
    /// Add a new expense
    Add(AddArgs),
    /// List expenses
    List(ListArgs),
    /// Show summaries
    Summary(SummaryArgs),
    /// Delete an expense by ID
    Delete(DeleteArgs),
    /// Edit an expense by ID
    Edit(EditArgs),
    /// Show stats for the current month
    Stats,
}

#[derive(Args)]
struct AddArgs {
    /// Category (e.g. food, shopping)
    category: String,
    /// Amount
    amount: f64,
    /// Optional note
    #[arg(long, short)]
    note: Option<String>,
    /// Date (YYYY-MM-DD), defaults to today
    #[arg(long, short)]
    date: Option<NaiveDate>,
}

#[derive(Args)]
struct ListArgs {
    /// Filter by month (YYYY-MM)
    #[arg(long)]
    month: Option<String>,
    /// Filter by date (YYYY-MM-DD)
    #[arg(long)]
    date: Option<NaiveDate>,
}

#[derive(Args)]
struct SummaryArgs {
    #[command(subcommand)]
    kind: SummaryKind,
}

#[derive(Subcommand)]
enum SummaryKind {
    /// Daily total
    Daily {
        /// Date (YYYY-MM-DD), defaults to today
        #[arg(long, short)]
        date: Option<NaiveDate>,
    },
    /// Weekly total (week starts Monday)
    Weekly,
    /// Monthly total
    Monthly {
        /// Month (YYYY-MM), defaults to current month
        #[arg(long, short)]
        month: Option<String>,
    },
    /// Breakdown by category
    Category {
        /// Month (YYYY-MM), defaults to current month
        #[arg(long, short)]
        month: Option<String>,
    },
}

#[derive(Args)]
struct DeleteArgs {
    /// Expense ID to delete
    id: i64,
}

#[derive(Args)]
struct EditArgs {
    /// Expense ID to edit
    id: i64,
    #[arg(long, short)]
    category: Option<String>,
    #[arg(long, short)]
    amount: Option<f64>,
    #[arg(long, short)]
    note: Option<String>,
    #[arg(long, short)]
    date: Option<NaiveDate>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            if let Err(e) = database::init() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Command::Add(args) => {
            let conn = database::open_or_fail();
            if let Err(e) = expense::add(
                &conn,
                &args.category,
                args.amount,
                args.note.as_deref(),
                args.date,
            ) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Command::List(args) => {
            let conn = database::open_or_fail();
            let range = if let Some(m) = args.month {
                Some(parse_month_or_exit(&m))
            } else {
                args.date.map(date_utils::daily_range)
            };
            match expense::list(&conn, range) {
                Ok(expenses) => expense::print_table(&expenses),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Command::Summary(args) => {
            let conn = database::open_or_fail();
            match args.kind {
                SummaryKind::Daily { date } => {
                    let d = date.unwrap_or_else(date_utils::today);
                    let (start, end) = date_utils::daily_range(d);
                    match summary::total_in_range(&conn, start, end) {
                        Ok(total) => {
                            println!("Daily summary for {}", d);
                            println!("  Total: ₹{:.2}", total);
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }

                SummaryKind::Weekly => {
                    let today = date_utils::today();
                    let (start, end) = date_utils::weekly_range(today);
                    match summary::total_in_range(&conn, start, end) {
                        Ok(total) => {
                            println!(
                                "Weekly summary ({} to {})",
                                start.format("%Y-%m-%d"),
                                (end - chrono::Duration::days(1)).format("%Y-%m-%d")
                            );
                            println!("  Total: ₹{:.2}", total);
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }

                SummaryKind::Monthly { month } => {
                    let (year, m) = month
                        .map(|s| parse_month_str_or_exit(&s))
                        .unwrap_or_else(|| {
                            let t = date_utils::today();
                            (t.year(), t.month())
                        });
                    let (start, end) = date_utils::monthly_range(year, m);
                    match summary::total_in_range(&conn, start, end) {
                        Ok(total) => {
                            println!(
                                "Monthly summary for {} {}",
                                date_utils::month_name(m),
                                year
                            );
                            println!("  Total: ₹{:.2}", total);
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }

                SummaryKind::Category { month } => {
                    let (year, m) = month
                        .map(|s| parse_month_str_or_exit(&s))
                        .unwrap_or_else(|| {
                            let t = date_utils::today();
                            (t.year(), t.month())
                        });
                    let (start, end) = date_utils::monthly_range(year, m);
                    match summary::by_category_in_range(&conn, start, end) {
                        Ok(rows) if rows.is_empty() => println!("No expenses found."),
                        Ok(rows) => {
                            let total: f64 = rows.iter().map(|(_, a)| a).sum();
                            println!(
                                "Category summary for {} {}",
                                date_utils::month_name(m),
                                year
                            );
                            println!("{:<20} {:>10}  {:>7}", "Category", "Amount", "Share");
                            println!("{}", "-".repeat(42));
                            for (cat, amt) in &rows {
                                println!(
                                    "{:<20} {:>10}  {:>6.1}%",
                                    cat,
                                    format!("₹{:.2}", amt),
                                    amt / total * 100.0
                                );
                            }
                            println!("{}", "-".repeat(42));
                            println!("{:<20} {:>10}", "Total", format!("₹{:.2}", total));
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        }

        Command::Delete(args) => {
            let conn = database::open_or_fail();
            if let Err(e) = expense::delete(&conn, args.id) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Command::Edit(args) => {
            if args.category.is_none()
                && args.amount.is_none()
                && args.note.is_none()
                && args.date.is_none()
            {
                eprintln!("Nothing to update. Provide --category, --amount, --note, or --date.");
                std::process::exit(1);
            }
            let conn = database::open_or_fail();
            if let Err(e) = expense::edit(
                &conn,
                args.id,
                args.category.as_deref(),
                args.amount,
                args.note.as_deref(),
                args.date,
            ) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Command::Stats => {
            let conn = database::open_or_fail();
            let today = date_utils::today();
            let (start, end) = date_utils::monthly_range(today.year(), today.month());
            println!(
                "Stats for {} {}",
                date_utils::month_name(today.month()),
                today.year()
            );
            println!("{}", "-".repeat(40));
            if let Err(e) = summary::print_stats(&conn, start, end) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn parse_month_or_exit(s: &str) -> (chrono::NaiveDateTime, chrono::NaiveDateTime) {
    match date_utils::parse_year_month(s) {
        Ok((year, month)) => date_utils::monthly_range(year, month),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_month_str_or_exit(s: &str) -> (i32, u32) {
    match date_utils::parse_year_month(s) {
        Ok(ym) => ym,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
