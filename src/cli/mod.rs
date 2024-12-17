pub mod git;
pub mod web;

use crate::core::web::{SearchParams, basic_search};
pub use crate::git::*;
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand, command};
use git::GitCommands;
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
    /// Seach google.
    #[command(about = "Search google.")]
    WebSearch {
        /// The search query that shows up in the google search bar.
        #[arg(required = true)]
        query: String,
        #[arg(long, default_value = "true", default_value_t = true)]
        open: bool,
    },
}

pub fn handle_commands(cli: &Cli) -> () {
    // println!("{:?}", cli.commands);
    match &cli.commands {
        Some(command) => match command {
            Commands::Git { com } => git::handle_commands(com),
            Commands::WebSearch { query, open } => basic_search(SearchParams::new(query), open),
        },
        None => println!("No command provided."),
    }
}

/// Returns the current DateTime object in the local timezone.
pub fn time_now() -> DateTime<Local> {
    let local = chrono::Local::now();
    local
}
