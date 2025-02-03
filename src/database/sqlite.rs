#![allow(unused)]
use chrono::{DateTime, Local, NaiveDateTime};
use diesel::{dsl::now, prelude::*};

use crate::database::{
    model::{DailyQuote, Quote},
    schema::quotes::quote,
};

use super::{
    DbResult,
    model::{NewDailyQuote, NewQuote, SearchEntry},
};

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
pub fn get_quote(id: i32) -> DbResult<Quote> {
    use crate::database::schema::quotes::dsl::*;

    let conn = &mut establish_connection()?;
    let result = quotes.find(id).first::<Quote>(conn);

    match result {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Get the most recent daily quote.
pub fn get_daily_quote() -> DbResult<Quote> {
    use crate::database::schema::daily_quotes::dsl::*;

    let conn = &mut establish_connection()?;
    let result = daily_quotes
        .order(time_stamp.desc())
        .first::<DailyQuote>(conn);

    match result {
        //  If there is a valid daily quote, check if it is from today.
        //  If it is not from today, get a new random quote and insert it as the daily quote.
        Ok(q) => {
            let current_date = chrono::Local::now().date_naive();
            if q.time_stamp.date() != current_date {
                let rand_quote = get_quote_random()?;
                let new_daily_quote = NewDailyQuote {
                    quote_id: rand_quote.id,
                    time_stamp: Local::now().naive_local(),
                };
                insert_daily_quote(new_daily_quote)?;
                Ok(rand_quote)
            } else {
                let new_quote = get_quote(q.quote_id)?;
                Ok(new_quote)
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

/// Get a search by its ID.
pub fn get_search(id: i32) -> DbResult<SearchEntry> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result = searches.find(id).first::<SearchEntry>(conn);

    match result {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Get a search by its query.
pub fn get_search_by_query(query: String) -> DbResult<SearchEntry> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result = searches.filter(query.eq(query)).first::<SearchEntry>(conn);

    match result {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Get a search by its query, website, and allintext.
pub fn get_search_by(
    query: String,
    site: Option<String>,
    allintext: Option<String>,
) -> DbResult<SearchEntry> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result = searches
        .filter(
            query
                .eq(query)
                .and(website.eq(site))
                .and(allintext.eq(allintext)),
        )
        .first::<SearchEntry>(conn);

    match result {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Get all search history items within a range.
///
/// If `from` is empty, all items before `to` are returned.
/// If `to` is empty, all items after `from` are returned.
/// If both are empty, all items are returned.
pub fn get_search_range(from: String, to: String) -> DbResult<Vec<SearchEntry>> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result: Result<Vec<SearchEntry>, diesel::result::Error>;

    if from.is_empty() && to.is_empty() {
        result = searches.load(conn);
    } else if from.is_empty() {
        result = searches.filter(time_stamp.le(to)).load(conn);
    } else if to.is_empty() {
        result = searches.filter(time_stamp.ge(from)).load(conn);
    } else {
        result = searches
            .filter(time_stamp.ge(from))
            .filter(time_stamp.le(to))
            .load(conn);
    }

    match result {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Insert a new search history item.
pub fn insert_search(new_search: SearchEntry) -> DbResult<()> {
    use crate::database::schema::searches::dsl::*;

    // println!("Inserting search: {:?}", new_search);
    let conn = &mut establish_connection()?;

    let search_res = searches.select(SearchEntry::as_select()).load(conn);

    let mut new_id = 0;

    if search_res.is_ok() {
        let search_vec = search_res.unwrap();

        if !search_vec.is_empty() {
            let last_search = search_vec.iter().last();
            if last_search.is_some() {
                let last_id = last_search.unwrap().id;
                new_id = last_id + 1;
            }
        }
    } else {
        return Err("Failed to get search history".into());
    }

    let search = SearchEntry {
        id: new_id,
        query: new_search.query.clone(),
        website: new_search.website.clone(),
        allintext: new_search.allintext.clone(),
        time_stamp: chrono::Local::now().naive_local(),
    };

    // println!("Inserting search: {:?}", search);

    let result = diesel::insert_into(searches).values(&search).execute(conn);

    match result {
        Ok(_) => Ok(()),
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

/// Delete a search history item matching its ID.
pub fn delete_search(id: i32) -> DbResult<()> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result = diesel::delete(searches.find(id)).execute(conn);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Delete search history items within a range.
///
/// If `from` is empty, all items before `to` are deleted.
/// If `to` is empty, all items after `from` are deleted.
/// If both are empty, all items are deleted.
pub fn delete_search_range(from: String, to: String) -> DbResult<()> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result: Result<usize, diesel::result::Error>;

    if from.is_empty() && to.is_empty() {
        result = diesel::delete(searches).execute(conn);
    } else if from.is_empty() {
        result = diesel::delete(searches.filter(time_stamp.le(to))).execute(conn);
    } else if to.is_empty() {
        result = diesel::delete(searches.filter(time_stamp.ge(from))).execute(conn);
    } else {
        result = diesel::delete(
            searches
                .filter(time_stamp.ge(from))
                .filter(time_stamp.le(to)),
        )
        .execute(conn);
    }

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}
