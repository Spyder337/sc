use clap::Parser;
pub mod cli;
pub mod core;
pub mod git;

pub use crate::cli::*;
pub use crate::core::sql::Database;
pub use crate::git::core::*;

fn main() {
    // let _m = cli::build_cli().get_matches();
    // println!("Matches: \n{:?}", _m)
    init();
    let cli = Cli::parse();
    handle_commands(&cli);
}
