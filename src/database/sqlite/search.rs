use std::fmt::format;

use chrono::{DateTime, Local, NaiveDateTime};
use diesel::{dsl::now, prelude::*};

use crate::database::sqlite::establish_connection;

use super::{DailyQuote, DbResult, NewDailyQuote, NewQuote, Quote, SearchEntry};

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
pub fn get_search_by_query(query_str: String) -> DbResult<Vec<SearchEntry>> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result = searches
        .filter(query.like(&format!("{}%", query_str)))
        .load::<SearchEntry>(conn);

    match result {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Get a search by its query, website, and allintext.
pub fn get_search_by(
    query_str: Option<String>,
    site_str: Option<String>,
    allintext_str: Option<String>,
) -> DbResult<Vec<SearchEntry>> {
    use crate::database::schema::searches::dsl::*;

    let conn = &mut establish_connection()?;
    let result;
    if query_str.is_some() && site_str.is_none() && allintext_str.is_none() {
        result = searches
            .filter(query.like(&format!("{}%", query_str.unwrap())))
            .load::<SearchEntry>(conn);
    } else if query_str.is_some() && site_str.is_none() && allintext_str.is_some() {
        result = searches
            .filter(
                query
                    .like(&format!("{}%", query_str.unwrap()))
                    .and(allintext.like(&format!("{}%", allintext_str.unwrap_or("".to_string())))),
            )
            .load(conn);
    } else if query_str.is_some() && site_str.is_some() && allintext_str.is_none() {
        result = searches
            .filter(
                query
                    .like(&format!("{}%", query_str.unwrap()))
                    .and(website.like(&format!("{}%", site_str.unwrap_or("".to_string())))),
            )
            .load(conn);
    } else if query_str.is_some() && site_str.is_some() && allintext_str.is_some() {
        result = searches
            .filter(
                query
                    .like(&format!("{}%", query_str.unwrap()))
                    .and(website.like(&format!("{}%", site_str.unwrap())))
                    .and(allintext.like(&format!("{}%", allintext_str.unwrap()))),
            )
            .load(conn);
    } else if site_str.is_some() && allintext_str.is_none() {
        result = searches
            .filter(website.like(&format!("{}%", site_str.unwrap())))
            .load(conn);
    } else if site_str.is_some() && allintext_str.is_some() {
        result = searches
            .filter(
                website
                    .like(&format!("{}%", site_str.unwrap()))
                    .and(allintext.like(&format!("{}%", allintext_str.unwrap()))),
            )
            .load(conn);
    } else if allintext_str.is_some() {
        result = searches
            .filter(allintext.like(&format!("{}%", allintext_str.unwrap())))
            .load(conn);
    } else {
        result = searches.load(conn);
    }
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
