use clap::Parser;
pub mod cli;
pub mod git;

pub use crate::cli::*;
pub use crate::git::core::*;

fn main() {
    // let _m = cli::build_cli().get_matches();
    // println!("Matches: \n{:?}", _m)
    let cli = Cli::parse();
    handle_commands(&cli);
}
