#![deny(missing_docs)]
//! # ShellCommander
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

use clap::Parser;
use commands::{ClapParser, CommandHandler, environment::core::Environment};
use directories::UserDirs;
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

/// Replace backslashes with forward slashes in a [`Path`].
pub fn sanitize_path(path_buf: &Path) -> PathBuf {
    path_buf.to_string_lossy().replace("\\", "/").into()
}

/// Replace backslashes with forwards slashes in a [`PathBuf`].
pub fn sanitize_pathbuf(path_buf: PathBuf) -> PathBuf {
    path_buf.to_string_lossy().replace("\\", "/").into()
}

/// Replace the '~' with the home directory
pub fn expand_home(path: &Path) -> PathBuf {
    let mut new_path: PathBuf = PathBuf::new();
    if path.starts_with("~") {
        let dirs = UserDirs::new().unwrap();
        let base_dir = dirs.home_dir().to_path_buf();
        let sub_path: PathBuf = path.iter().skip(1).collect();
        new_path.push(base_dir);
        new_path.push(sub_path);
    }
    new_path
}

/// Replace the '~' with the home directory and sanitize the path.
pub fn expand_sanitized_home(path: &Path) -> PathBuf {
    sanitize_path(&expand_home(path))
}

fn main() -> Result<()> {
    let parser = ClapParser::parse();
    parser.handle()
}
