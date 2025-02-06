pub mod core;

use clap::Subcommand;

use core::{SearchParams, basic_search, query_string_builder};

use crate::{
    commands::time_now,
    database::{self, model::SearchEntry},
};

use super::CommandHandler;

/// A set of web utilities.
#[derive(Debug, Subcommand)]
pub(crate) enum WebCommands {
    /// Search google for a query.
    Search {
        /// Search query.
        query: String,
        /// Site to restrict search to.
        #[arg(short = None, long)]
        site: Option<String>,
        /// Search for text in the page.
        #[arg(short = None, long)]
        allintext: Option<String>,
        /// Return results in JSON format.
        #[arg(short = None, long)]
        json: Option<bool>,
    },
    /// View search history.
    History {
        #[command(subcommand)]
        command: HistoryCommands,
    },
}

impl CommandHandler for WebCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            WebCommands::Search {
                query,
                site,
                allintext,
                json,
            } => {
                // println!("Searching for: {}", query);
                let query_string = query_string_builder(query, site, allintext);
                let search = SearchParams::new(&query_string);

                // println!("Search Params: {:?}", search);

                let search_res = basic_search(search, &(json.unwrap_or(false)));

                if search_res.is_err() {
                    return Err(search_res.err().unwrap());
                }

                let new_search = SearchEntry {
                    id: 0,
                    query: query.clone(),
                    website: site.clone(),
                    allintext: allintext.clone(),
                    time_stamp: time_now(),
                };

                // println!("Search Object: {:?}", new_search);

                let res = database::sqlite::insert_search(new_search);

                if res.is_err() {
                    return Err(res.err().unwrap());
                }
                Ok(())
            }
            WebCommands::History { command } => command.handle(),
        }
    }
}

/// Commands for interacting with search history.
#[derive(Debug, Subcommand)]
pub(crate) enum HistoryCommands {
    List {
        /// End date to filter searches.
        #[arg(short = None, long)]
        to: Option<String>,
        /// Start date to filter searches.
        #[arg(short = None, long)]
        from: Option<String>,
    },
    Clear {
        /// End date to filter searches.
        #[arg(short = None, long)]
        to: Option<String>,
        /// Start date to filter searches.
        #[arg(short = None, long)]
        from: Option<String>,
        /// Site to filter searches.
        #[arg(short = None, long)]
        site: Option<String>,
    },
    Search {
        /// Search query.
        #[arg(short = None, long)]
        query: String,
        /// Site to restrict search to.
        #[arg(short = None, long)]
        site: Option<String>,
        /// Search for text in the page.
        #[arg(short = None, long)]
        allintext: Option<String>,
    },
}

impl CommandHandler for HistoryCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            HistoryCommands::List { to, from } => history_list(to.clone(), from.clone()),
            HistoryCommands::Clear { to, from, site } => {
                history_clear(to.clone(), from.clone(), site.clone())
            }
            HistoryCommands::Search {
                query,
                site,
                allintext,
            } => history_search(query.clone(), site.clone(), allintext.clone()),
        }
    }
}

/// List search history.
fn history_list(to: Option<String>, from: Option<String>) -> crate::Result<()> {
    let res = database::sqlite::get_search_range(from.unwrap_or_default(), to.unwrap_or_default());
    if res.is_err() {
        return Err(res.err().unwrap());
    }
    for search in res.unwrap() {
        println!("{}", search);
    }
    Ok(())
}

/// Clear search history.
fn history_clear(
    to: Option<String>,
    from: Option<String>,
    _site: Option<String>,
) -> crate::Result<()> {
    let res =
        database::sqlite::delete_search_range(from.unwrap_or_default(), to.unwrap_or_default());
    if res.is_err() {
        return Err(res.err().unwrap());
    }
    Ok(())
}

/// Search history. 
/// 
/// # Arguments
/// query: The search query to search for.
/// site: The site to filter searches by.
/// allintext: The required text to filter searches by.
fn history_search(
    query: String,
    site: Option<String>,
    allintext: Option<String>,
) -> crate::Result<()> {
    let res = database::sqlite::get_search_by(query, site, allintext);
    if res.is_err() {
        return Err(res.err().unwrap());
    }
    println!("{:?}", res.unwrap());
    Ok(())
}
