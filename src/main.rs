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
    // set_git_dir("~/Code");
    // set_git_author("Spyder337");
    // set_git_email("owsley.wood@gmail.com");
    init();
    let cli = Cli::parse();
    handle_commands(&cli);
}
