use clap::{Parser, Subcommand};

/// The top-level CLI structure
#[derive(Parser)]
#[command(name = "mydocker")]
#[command(about = "A minimal Docker CLI in Rust")]
pub struct Cli {
    /// The main command to execute
    #[command(subcommand)]
    pub command: Command,
}

/// Enum for top-level commands
#[derive(Subcommand)]

pub enum Command {
    /// List resources
    List {
        #[command(subcommand)]
        list_command: ListCommands,
    },
}

/// Subcommands for listing resources
#[derive(Subcommand)]
pub enum ListCommands {
    // List containers
    Containers{
        // Include stopped containers
        #[arg(short, long)]
        all: bool,
    },
    // Images,
}