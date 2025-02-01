#![allow(dead_code)]

pub mod model;
mod schema;
pub mod sqlite;

/// A wrapper for [std::result::Result] with a [Box]ed [std::error::Error].
pub type DbResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn generate_dotenv() -> DbResult<()> {
    use crate::ENV;
    use std::fs::File;
    use std::io::Write;
    let env = ENV.lock().unwrap();

    let mut file = File::create(".env")?;
    writeln!(file, "DATABASE_URL={}", env.conn_str)?;

    Ok(())
}
