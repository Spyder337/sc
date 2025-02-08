#![allow(unused)]

mod quote;
mod search;

use std::fmt::format;

use chrono::{DateTime, Local, NaiveDateTime};
use diesel::{dsl::now, prelude::*};

use super::{DailyQuote, DbResult, NewDailyQuote, NewQuote, Quote, SearchEntry};

pub use quote::*;
pub use search::*;

/// Create a connection to the sqlite database.
///
/// The connection string is stored in the `ENV` global variable.
fn establish_connection() -> DbResult<SqliteConnection> {
    let env = crate::ENV.lock().unwrap();
    let conn = SqliteConnection::establish(&env.conn_str)?;
    Ok(conn)
}

/// Generate a random number from \[0, `max`\].
fn random_i32(max: i32) -> i32 {
    use rand::Rng;

    let mut rng = rand::rng();
    rng.random_range(0..max)
}
