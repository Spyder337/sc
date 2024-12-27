use crate::database::schema::{quotes, searches};
use diesel::prelude::*;

/// A google search history item.
#[derive(Debug, Queryable, AsChangeset, Selectable, Clone)]
#[diesel(table_name = searches)]
pub struct Search {
    pub id: i32,
    pub query: String,
    pub website: Option<String>,
    pub allintext: Option<String>,
    pub time_stamp: String,
}

/// A new google search history item.
///
/// This struct is used to insert a new search history item into the database.
#[derive(Insertable)]
#[diesel(table_name = searches)]
pub struct NewSearch {
    pub query: String,
    pub website: Option<String>,
    pub allintext: Option<String>,
    pub time_stamp: String,
}

/// A quote.
#[derive(Debug, Queryable, AsChangeset, Selectable, Clone)]
#[diesel(table_name = quotes)]
pub struct Quote {
    pub id: i32,
    pub quote: String,
    pub author: String,
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
pub struct DailyQuote {
    pub id: i32,
    pub quote_id: i32,
    pub time_stamp: chrono::NaiveDateTime,
}

/// A new daily quote.
///
/// This struct is used to insert a new daily quote into the database.
pub struct NewDailyQuote {
    pub quote_id: i32,
    pub time_stamp: chrono::NaiveDateTime,
}
