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
use quotes::QuoteCommands;
use web::WebCommands;

pub use environment::core::{Environment, time_now};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Web {
        #[command(subcommand)]
        command: WebCommands,
    },
    Git {
        #[command(subcommand)]
        command: GitCommands,
    },
    Env {
        #[command(subcommand)]
        command: EnvCommands,
    },
    Quote {
        #[command(subcommand)]
        command: QuoteCommands,
    },
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
                print!("{}", greeting::welcome_msg());
                Ok(())
            }
        }
    }
}
