use crate::Result;
use crate::database::model::{NewDailyQuote, NewQuote, Quote};
use crate::database::sqlite::{
    get_daily_quote, get_quote, get_quote_random, get_quotes, insert_daily_quote, insert_quote,
};

pub fn add_quote(quote: &str, author: &str) -> Result<()> {
    let new_quote = NewQuote {
        quote: quote.to_string(),
        author: author.to_string(),
    };
    let res = insert_quote(new_quote);
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}

pub fn generate_daily_quote() -> Result<()> {
    let quote = get_quote_random();

    if quote.is_err() {
        return Err(quote.err().unwrap().to_string().into());
    }

    let daily_quote = NewDailyQuote {
        quote_id: quote.unwrap().id,
    };
    let res = insert_daily_quote(daily_quote);

    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}

pub fn get_quote_by_id(id: i32) -> Result<Quote> {
    let res = get_quote(id);

    match res {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

pub fn get_quotes_all() -> Result<Vec<Quote>> {
    let res = get_quotes();

    match res {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}

pub fn get_daily() -> Result<Quote> {
    let res = get_daily_quote();

    match res {
        Ok(q) => Ok(q),
        Err(e) => Err(e.to_string().into()),
    }
}
