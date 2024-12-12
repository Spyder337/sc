use clap::Parser;
pub mod cli;

pub use crate::cli::*;

fn main() {
    // let _m = cli::build_cli().get_matches();
    // println!("Matches: \n{:?}", _m)
    let cli = Cli::parse();
    handle_commands(&cli);
}
