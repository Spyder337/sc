use crate::database::schema::{daily_quotes, quotes, searches};
use chrono::NaiveDateTime;
use diesel::prelude::*;

/// A google search history item.
#[derive(Debug, Queryable, AsChangeset, Selectable, Clone, Insertable)]
#[diesel(table_name = searches)]
pub struct SearchEntry {
    pub id: i32,
    pub query: String,
    pub website: Option<String>,
    pub allintext: Option<String>,
    #[diesel(sql_type = Timestamp)]
    pub time_stamp: NaiveDateTime,
}

impl std::fmt::Display for SearchEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_fmt(format_args!("Query: {}\n", self.query));
        if let Some(website) = &self.website {
            let _ = f.write_fmt(format_args!("Website: {}\n", website));
        }
        if let Some(all_text) = &self.allintext {
            let _ = f.write_fmt(format_args!("All in text: {}\n", all_text));
        }
        let _ = f.write_fmt(format_args!("Time: {}\n", self.time_stamp));
        f.write_fmt(format_args!(
            "Query String: {}\n",
            crate::commands::web::core::query_string_builder(
                &self.query,
                &self.website,
                &self.allintext,
            )
        ))
    }
}

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
        f.write_str(format!("{}\n\t- {}", self.quote, self.author).as_str())
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
