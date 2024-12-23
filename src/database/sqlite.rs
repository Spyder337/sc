use diesel::prelude::*;

use crate::database::model::Quote;

use super::DbResult;

fn establish_connection() -> DbResult<SqliteConnection> {
    let conn = SqliteConnection::establish(&crate::ENV.lock().unwrap().conn_str)?;
    Ok(conn)
}

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

pub fn get_quote(id: i32) -> DbResult<Quote> {
    use crate::database::schema::quotes::dsl::*;

    let conn = &mut establish_connection()?;
    let result = quotes.find(id).first::<Quote>(conn);

    match result {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}
