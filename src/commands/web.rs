use clap::Subcommand;

use crate::{
    database::{self, model::NewSearch},
    web::{SearchParams, basic_search, query_string_builder},
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
                let query_string = query_string_builder(query, site, allintext);
                let search = SearchParams::new(&query_string);
                let new_search = NewSearch {
                    query: query.clone(),
                    website: site.clone(),
                    allintext: allintext.clone(),
                };
                let res = database::sqlite::insert_search(new_search);

                if res.is_err() {
                    return Err(res.err().unwrap());
                }

                basic_search(search, &(!json.unwrap_or(false)))
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

fn history_list(to: Option<String>, from: Option<String>) -> crate::Result<()> {
    let res = database::sqlite::get_search_range(from.unwrap_or_default(), to.unwrap_or_default());
    if res.is_err() {
        return Err(res.err().unwrap());
    }
    for search in res.unwrap() {
        println!("{:?}", search);
    }
    Ok(())
}

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
