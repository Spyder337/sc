#![allow(dead_code)]

use std::path::{Path, PathBuf};

pub mod models;
mod schema;
pub mod sqlite;

use diesel_migrations::MigrationHarness;
pub use models::{
    quote::{DailyQuote, NewDailyQuote, NewQuote, Quote},
    search::SearchEntry,
};

/// A wrapper for [std::result::Result] with a [Box]ed [std::error::Error].
pub type DbResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn generate_dotenv() -> DbResult<()> {
    use crate::ENV;
    use std::fs::File;
    use std::io::Write;
    let env = ENV.lock().unwrap();
    let path = PathBuf::from(env.conn_str.as_str());

    let mut file = File::create(".env")?;
    write!(file, "DATABASE_URL={}", path.to_str().unwrap())?;

    Ok(())
}

pub fn run_migrations() -> DbResult<()> {
    use diesel_migrations::EmbeddedMigrations;
    const MIGRATIONS: EmbeddedMigrations = diesel_migrations::embed_migrations!("./migrations");

    let mut conn = sqlite::establish_connection()?;
    let res = conn.run_pending_migrations(MIGRATIONS);
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub fn init_database() -> DbResult<()> {
    if !PathBuf::from(".env").exists() {
        generate_dotenv()?;
    }
    generate_dotenv()?;
    run_migrations()
}
