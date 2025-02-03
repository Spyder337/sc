#![deny(missing_docs)]
//! # ShellCommander
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

use clap::Parser;
use commands::{ClapParser, CommandHandler, environment::core::Environment};
use lazy_static::lazy_static;

mod colors;
mod commands;
mod database;

/// Wrapper type for std::result::Result.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

lazy_static! {
    /// The application directory.
    pub static ref APP_DIR: PathBuf = {
        let base = directories::BaseDirs::new().unwrap();
        sanitize_path(&base.data_dir().join("ShellCommander"))
    };
    /// The path to the configuration file.
    pub static ref CONFIG_FILE: PathBuf = sanitize_path(&APP_DIR.join("config.toml"));
    /// The path to the SQLite database file.
    pub static ref SQL_FILE: PathBuf = sanitize_path(&APP_DIR.join("database.db"));
    /// The environment settings.
    pub static ref ENV: Mutex<Environment> = Mutex::new(Environment::load());
    /// Color codes for the terminal.
    pub static ref COLORS: HashMap<&'static str, &'static str> = colors::colors_init();
}

/// Replace backslashes with forward slashes in a path.
pub fn sanitize_path(path_buf: &Path) -> PathBuf {
    path_buf.to_string_lossy().replace("\\", "/").into()
}

fn main() -> Result<()> {
    let parser = ClapParser::parse();
    parser.handle()
}
