pub mod completions;
pub mod environment;
pub mod git;
pub mod greeting;
pub mod quotes;
pub mod web;

use clap::{Parser, Subcommand};

use completions::CompletionCommands;
use environment::EnvCommands;
use git::GitCommands;
use quotes::{QuoteCommands, core::get_daily};
use web::WebCommands;

pub use environment::core::{Environment, time_now};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Search google for a query or view search history.
    Web {
        #[command(subcommand)]
        command: WebCommands,
    },
    /// Manage a git repository.
    Git {
        #[command(subcommand)]
        command: GitCommands,
    },
    /// Manage the environment.
    Env {
        #[command(subcommand)]
        command: EnvCommands,
    },
    /// Add and get quotes from the database.
    Quote {
        #[command(subcommand)]
        command: QuoteCommands,
    },
    /// Generate shell completions for the specified shell.
    Completions {
        #[command(subcommand)]
        command: CompletionCommands,
    },
    /// Message of the day.
    Welcome,
}

/// A trait that indicates a struct encapsulates a command.
pub trait CommandHandler {
    /// Parse the command and args, returning an error on failure.
    fn handle(&self) -> crate::Result<()>;
}

/// A set of command line utilities.
#[derive(Debug, Parser)]
pub struct ClapParser {
    #[command(subcommand)]
    pub command: Commands,
}

impl CommandHandler for ClapParser {
    fn handle(&self) -> crate::Result<()> {
        match &self.command {
            Commands::Web { command } => command.handle(),
            Commands::Git { command } => command.handle(),
            Commands::Env { command } => command.handle(),
            Commands::Quote { command } => command.handle(),
            Commands::Completions { command } => command.handle(),
            Commands::Welcome => {
                println!("{}", greeting::welcome_msg());
                println!("{}", get_daily().unwrap());
                Ok(())
            }
        }
    }
}
