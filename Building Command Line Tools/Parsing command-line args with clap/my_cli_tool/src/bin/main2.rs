use clap::Parser;

/// Simple program to greet a person
#[derive(Parser)]
#[command(name = "greeter")]
#[command(about = "A simple CLI tool to greet a person", long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Customize the greeting message
    #[arg(short, long, default_value_t = String::from("Hello"))]
    greeting: String,

    /// Print the greeting in uppercase
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    uppercase: bool,
}

fn main() {
    let args: Cli = Cli::parse();

    let mut message: String = format!("{} {}", args.greeting, args.name);

    if args.uppercase {
        message = message.to_uppercase();
    }

    println!("{}", message);
}