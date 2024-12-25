use clap::Subcommand;

use crate::web::{SearchParams, basic_search, query_string_builder};

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
        open: bool,
    },
}

impl CommandHandler for HistoryCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            HistoryCommands::List { to, from } => todo!(),
            HistoryCommands::Clear { to, from, site } => todo!(),
            HistoryCommands::Search {
                query,
                site,
                allintext,
                open,
            } => todo!(),
        }
    }
}
