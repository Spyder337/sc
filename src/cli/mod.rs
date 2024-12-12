pub mod git;

pub use crate::git::*;
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand, command};
#[derive(Parser, Debug)]
#[command(version, about = "Quality of life commands.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Git repo interactions.")]
    Git {
        #[command(subcommand)]
        com: GitCommands,
    },
    #[command(about = "Search google.")]
    WebSearch {},
}

pub fn handle_commands(cli: &Cli) -> () {
    println!("{:?}", cli.commands);
    match &cli.commands {
        Some(command) => match command {
            Commands::Git { com } => git::handle_commands(com),
            Commands::WebSearch {} => todo!(),
        },
        None => println!("No command provided."),
    }
}

/// Returns the current DateTime object in the local timezone.
pub fn time_now() -> DateTime<Local> {
    let local = chrono::Local::now();
    local
}
