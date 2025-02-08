use std::fmt::format;

use chrono::{DateTime, Local, NaiveDateTime};
use diesel::{dsl::now, prelude::*};

use crate::database::sqlite::{establish_connection, random_i32};

use super::{DailyQuote, DbResult, NewDailyQuote, NewQuote, Quote, SearchEntry};

/// Get all quotes from the database.
pub fn get_quotes() -> DbResult<Vec<Quote>> {
    use crate::database::schema::quotes::dsl::*;

    let conn = &mut establish_connection()?;
    let result = quotes.select(Quote::as_select()).load(conn);

    let mut res: Vec<Quote> = vec![];
    if let Ok(items) = &result {
        if items.is_empty() {
            return Err("No quotes found".into());
        }
        println!("Found {} quotes", items.len());
        for item in items {
            res.push(item.clone());
        }
    }
    Ok(res)
}

/// Get a quote by its ID.
pub fn get_quote(quote_id: i32) -> DbResult<Quote> {
    use crate::database::schema::quotes::dsl::*;

    let conn = &mut establish_connection()?;
    let result = quotes.find(quote_id).first::<Quote>(conn);
    match result {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Get the most recent daily quote.
pub fn get_daily_quote() -> DbResult<Quote> {
    use crate::database::schema::daily_quotes::dsl::*;

    let conn = &mut establish_connection()?;
    let result = daily_quotes.order(id.desc()).first::<DailyQuote>(conn);

    match result {
        //  If there is a valid daily quote, check if it is from today.
        //  If it is not from today, get a new random quote and insert it as the daily quote.
        Ok(q) => {
            let current_date = chrono::Local::now().date_naive();
            let recent_date = q.time_stamp.date();
            if recent_date == current_date {
                let new_quote = get_quote(q.quote_id)?;
                Ok(new_quote)
            } else {
                let rand_quote = get_quote_random()?;
                let new_daily_quote = NewDailyQuote {
                    quote_id: rand_quote.id,
                    time_stamp: Local::now().naive_local(),
                };
                insert_daily_quote(new_daily_quote)?;
                Ok(rand_quote)
            }
        }
        //  If there is no daily quote, get a new random quote and insert it as the daily quote.
        Err(e) => {
            let rand_quote = get_quote_random()?;
            let new_daily_quote = NewDailyQuote {
                quote_id: rand_quote.id,
                time_stamp: Local::now().naive_local(),
            };
            insert_daily_quote(new_daily_quote)?;
            Ok(rand_quote)
        }
    }
}

/// Gets a random quote from the database.
pub fn get_quote_random() -> DbResult<Quote> {
    use crate::database::schema::quotes::dsl::*;

    let conn = &mut establish_connection()?;
    let result = quotes.select(Quote::as_select()).load::<Quote>(conn);

    match result {
        Ok(q) => {
            let index = random_i32(q.len() as i32);
            Ok(q[index as usize].clone())
        }
        Err(e) => Err(e.to_string().into()),
    }
}

/// Insert a new quote.
pub fn insert_quote(new_quote: NewQuote) -> DbResult<()> {
    use crate::database::schema::quotes::dsl::*;

    let conn = &mut establish_connection()?;

    let mut new_id = 0;

    // Get the last quote ID.
    let quote_res = quotes.select(Quote::as_select()).load(conn);

    // If there are quotes in the database, get the last quote ID.
    if quote_res.is_ok() {
        let quotes_vec = quote_res.unwrap();

        if !quotes_vec.is_empty() {
            let last_quote = quotes_vec.iter().last();
            // println!("Last quote: {:?}", last_quote);
            if last_quote.is_some() {
                let last_id = last_quote.unwrap().id;
                new_id = last_id + 1;
            }
        }
    }

    let final_quote = Quote {
        id: new_id,
        quote: new_quote.quote.clone(),
        author: new_quote.author.clone(),
    };

    let result = diesel::insert_into(quotes)
        .values(&final_quote)
        .execute(conn);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Insert a new daily quote.
pub fn insert_daily_quote(new_daily_quote: NewDailyQuote) -> DbResult<()> {
    use crate::database::schema::daily_quotes::dsl::*;

    let conn = &mut establish_connection()?;

    let mut new_id = 0;

    let daily_res = daily_quotes.select(DailyQuote::as_select()).load(conn);

    if daily_res.is_ok() {
        let daily_vec = daily_res.unwrap();

        if !daily_vec.is_empty() {
            let last_daily = daily_vec.iter().last();
            if last_daily.is_some() {
                let last_id = last_daily.unwrap().id;
                new_id = last_id + 1;
            }
        }
    }

    let final_daily_quote = DailyQuote {
        id: new_id,
        quote_id: new_daily_quote.quote_id,
        time_stamp: new_daily_quote.time_stamp,
    };

    let result = diesel::insert_into(daily_quotes)
        .values(&final_daily_quote)
        .execute(conn);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Delete a quote matching its ID.
pub fn delete_quote(id: i32) -> DbResult<()> {
    use crate::database::schema::quotes::dsl::*;

    let conn = &mut establish_connection()?;
    let result = diesel::delete(quotes.find(id)).execute(conn);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}
