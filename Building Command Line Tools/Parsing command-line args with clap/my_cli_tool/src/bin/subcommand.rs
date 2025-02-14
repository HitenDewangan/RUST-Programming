// A CLI tool with subcommands
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[command(name = "greeter")]
#[command(about = "A simple CLI tool to greet a person", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello to someone
    Hello {
        #[arg(short, long)]
        name: String,
    },
    /// Say goodbye to someone
    Goodbye {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Hello { name } => {
            println!("Hello, {}!", name);
        }
        Commands::Goodbye { name } => {
            println!("Goodbye, {}!", name);
        }
    }
}