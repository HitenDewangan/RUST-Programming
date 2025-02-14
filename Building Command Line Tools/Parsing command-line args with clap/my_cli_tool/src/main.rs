use clap::Parser;

/// Simple program to greet a person
#[derive(Parser)]
#[command(name = "greeter")]
#[command(about = "A simple CLI tool to greet a person", long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args: Cli = Cli::parse();
    println!("Hello, {}!", args.name);
}