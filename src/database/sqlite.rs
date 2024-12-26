#![allow(unused)]
use diesel::prelude::*;

use crate::database::model::Quote;

use super::{
    DbResult,
    model::{NewQuote, NewSearch, Search},
};

fn establish_connection() -> DbResult<SqliteConnection> {
    let conn = SqliteConnection::establish(&crate::ENV.lock().unwrap().conn_str)?;
    Ok(conn)
}

/// Generate a random number from \[0, `max`\].
fn random_i32(max: i32) -> i32 {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
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
pub fn get_search(id: i32) -> DbResult<Search> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result = searches.find(id).first::<Search>(conn);

    match result {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Get a search by its query.
pub fn get_search_by_query(query: String) -> DbResult<Search> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result = searches.filter(query.eq(query)).first::<Search>(conn);

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
) -> DbResult<Search> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result = searches
        .filter(
            query
                .eq(query)
                .and(website.eq(site))
                .and(allintext.eq(allintext)),
        )
        .first::<Search>(conn);

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
pub fn get_search_range(from: String, to: String) -> DbResult<Vec<Search>> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result: Result<Vec<Search>, diesel::result::Error>;

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
pub fn insert_search(new_search: NewSearch) -> DbResult<()> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result = diesel::insert_into(searches)
        .values(&new_search)
        .execute(conn);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Insert a new quote.
pub fn insert_quote(new_quote: NewQuote) -> DbResult<()> {
    use crate::database::schema::quotes::dsl::*;

    let conn = &mut establish_connection()?;
    let result = diesel::insert_into(quotes).values(&new_quote).execute(conn);

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
