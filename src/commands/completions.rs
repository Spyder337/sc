use std::{
    fs::File,
    io::{BufWriter, Write},
};

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
    Generate {
        shell: Shells,
        #[arg(short, long)]
        out_path: Option<String>,
    },
}

impl CommandHandler for CompletionCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            CompletionCommands::Generate { shell, out_path } => {
                completion_generate(shell, out_path);
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

/// Generate completions for the specified shell.
/// 
/// # Arguments
/// shell: The shell to generate completions for.
/// out_path: The path to write the completions to. If None, write to stdout.
pub(crate) fn completion_generate(shell: &Shells, out_path: &Option<String>) {
    let mut buffer: Box<dyn Write>;

    if let Some(path) = out_path {
        buffer = Box::new(BufWriter::new(File::create(path).unwrap()));
    } else {
        buffer = Box::new(std::io::stdout());
    }

    match shell {
        Shells::Nushell => generate(
            Nushell,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut buffer,
        ),
        Shells::Bash => generate(
            Bash,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut buffer,
        ),
        Shells::Elvish => generate(
            Elvish,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut buffer,
        ),
        Shells::Fish => generate(
            Fish,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut buffer,
        ),
        Shells::PowerShell => generate(
            PowerShell,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut buffer,
        ),
        Shells::Zsh => generate(
            Zsh,
            &mut ClapParser::command(),
            "ShellCommander",
            &mut buffer,
        ),
    }
}
