#![deny(missing_docs)]
//! # ShellCommander
use std::{path::PathBuf, sync::Mutex};

use clap::Parser;
use commands::{ClapParser, CommandHandler};
use environment::Environment;
use lazy_static::lazy_static;

mod commands;
mod database;
mod environment;
mod web;

/// Wrapper type for std::result::Result.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

lazy_static! {
    /// The application directory.
    pub static ref APP_DIR: PathBuf = {
        let base = directories::BaseDirs::new().unwrap();
        base.data_dir().join("ShellCommander").into()
    };
    /// The path to the configuration file.
    pub static ref CONFIG_FILE: PathBuf = APP_DIR.join("config.toml");
    /// The path to the SQLite database file.
    pub static ref SQL_FILE: PathBuf = APP_DIR.join("database.db");
    /// The environment settings.
    pub static ref ENV: Mutex<Environment> = Mutex::new(Environment::load());
}

fn main() -> Result<()> {
    let parser = ClapParser::parse();
    parser.handle()
}
