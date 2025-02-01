mod env;
mod git;
mod web;
mod quotes;

use clap::{Parser, Subcommand};

use env::EnvCommands;
use git::GitCommands;
use web::WebCommands;

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
    Quote,
    Completions,
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
            Commands::Quote => {
                println!("Quote command not implemented.");
                Ok(())
            }
            Commands::Completions => {
                println!("Completions command not implemented.");
                Ok(())
            }
            Commands::Welcome => {
                println!("{}", crate::greeting::welcome_msg());
                Ok(())
            }
        }
    }
}
