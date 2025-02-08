use crate::{
    colors::apply_color,
    database::schema::{daily_quotes, quotes},
};
use diesel::prelude::*;

/// A quote.
#[derive(Debug, Queryable, AsChangeset, Selectable, Clone, Insertable)]
#[diesel(table_name = quotes)]
pub struct Quote {
    pub id: i32,
    pub quote: String,
    pub author: String,
}

impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}\n\t- {}",
            apply_color("magenta_bright", &self.quote),
            apply_color("green_bright", &self.author)
        ))
    }
}

/// A new quote.
///
/// This struct is used to insert a new quote into the database.
#[derive(Insertable)]
#[diesel(table_name = quotes)]
pub struct NewQuote {
    pub quote: String,
    pub author: String,
}

/// A daily quote.
#[derive(Debug, Queryable, AsChangeset, Selectable, Clone, Insertable)]
#[diesel(table_name = daily_quotes)]
pub struct DailyQuote {
    pub id: i32,
    pub quote_id: i32,
    pub time_stamp: chrono::NaiveDateTime,
}

/// A new daily quote.
///
/// This struct is used to insert a new daily quote into the database.
#[derive(Insertable)]
#[diesel(table_name = daily_quotes)]
pub struct NewDailyQuote {
    pub quote_id: i32,
    pub time_stamp: chrono::NaiveDateTime,
}
