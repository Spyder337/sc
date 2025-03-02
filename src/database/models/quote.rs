use crate::{
    Colorize,
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
        let mut quote_str = String::new();
        let max_chars = 80;
        let mut char_cnt = 0;
        let words = self.quote.split_whitespace();
        for word in words {
            if char_cnt + word.len() > max_chars {
                quote_str.push_str("\n");
                char_cnt = 0;
            }
            quote_str.push_str(word);
            quote_str.push_str(" ");
            char_cnt += word.len();
        }
        f.write_str(&format!(
            "{}\n\t- {}",
            &quote_str.magenta_bright(),
            &self.author.green_bright()
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
