use clap::Parser;

#[derive(Parser)]
#[command(
    name = "ept",
    version,
    about = "A rust based high perfomance simple expense tracker CLI.",
    long_about = None
)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    println!("File: {}", args.file)
}