use clap::{CommandFactory, Subcommand};
use clap_complete::{
    aot::{Bash, Elvish, Fish, PowerShell, Zsh},
    generate,
};
use clap_complete_nushell::Nushell;

use crate::commands::ClapParser;

use super::CommandHandler;

/// Completions for different shells.
#[derive(Debug, Subcommand)]
pub enum CompletionCommands {
    /// Generate completions for a shell.
    Generate { shell: Shells },
}

impl CommandHandler for CompletionCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            CompletionCommands::Generate { shell } => {
                completion_generate(shell);
                Ok(())
            }
        }
    }
}

/// Possible shells to generate completions for.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Shells {
    Nushell,
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

pub(crate) fn completion_generate(shell: &Shells) -> () {
    match shell {
        Shells::Nushell => generate(
            Nushell,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut std::io::stdout(),
        ),
        Shells::Bash => generate(
            Bash,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut std::io::stdout(),
        ),
        Shells::Elvish => generate(
            Elvish,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut std::io::stdout(),
        ),
        Shells::Fish => generate(
            Fish,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut std::io::stdout(),
        ),
        Shells::PowerShell => generate(
            PowerShell,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut std::io::stdout(),
        ),
        Shells::Zsh => generate(
            Zsh,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut std::io::stdout(),
        ),
    }
}
