use clap::Parser;
pub mod cli;
pub mod core;
pub mod git;

pub use crate::cli::*;
pub use crate::core::sql::Database;
pub use crate::git::core::*;

fn main() {
    // display_git_info();
    let cli = Cli::parse();
    handle_commands(&cli);
}
