use crate::database::schema::{daily_quotes, quotes, searches};
use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::schema::searches::query;

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

impl ToString for SearchEntry {
    fn to_string(&self) -> String {
        let mut s = format!("Query: {}\n", self.query);
        if let Some(website) = &self.website {
            s.push_str(&format!("Website: {}\n", website));
        }
        if let Some(allintext) = &self.allintext {
            s.push_str(&format!("All in text: {}\n", allintext));
        }
        s.push_str(&format!("Time: {}\n", self.time_stamp));
        s.push_str(&format!(
            "Query String: {}\n",
            crate::commands::web::core::query_string_builder(
                &self.query,
                &self.website,
                &self.allintext,
            )
        ));
        s
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

impl ToString for Quote {
    fn to_string(&self) -> String {
        format!("{}\n\t- {}", self.quote, self.author)
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
