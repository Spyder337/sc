use clap::{Subcommand, arg};

use crate::{
    database::sqlite::get_quote_random,
    quote::{add_quote, get_daily, get_quote_by_id, get_quotes_all},
};

use super::CommandHandler;

//  TODO: Implement the commands for calling the functions.

#[derive(Debug, Subcommand)]
pub(crate) enum QuoteCommands {
    /// Add a new quote to the database.
    Add {
        #[arg(short = 'a', long)]
        author: Option<String>,
        #[arg(short = 'q', long)]
        quote: Option<String>,
    },
    /// Returns the daily quote.
    ///
    /// If the daily quote does not exist, a new one is generated.
    Daily,
    /// Get a single quote by its ID or all quotes as a vector.
    Get {
        /// The ID of the quote to get.
        #[arg(short = None, long)]
        id: Option<i32>,
    },
    /// Get a random quote from the database.
    Random,
}

impl CommandHandler for QuoteCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            //  TODO: Implement a menu and argument options for adding a quote.
            QuoteCommands::Add { author, quote } => {
                if author.is_none() && quote.is_none() {
                    println!("Please provide a quote and author.\n Use --help for usage");
                    Ok(())
                } else if author.is_none() || quote.is_none() {
                    println!("Please provide an author or a quote.\n Use --help for usage.");
                    Ok(())
                } else {
                    let res = add_quote(quote.as_ref().unwrap(), author.as_ref().unwrap());
                    match res {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.to_string().into()),
                    }
                }
            }
            //  TODO: Return the current daily quote or generate a new one.
            QuoteCommands::Daily => {
                let daily = get_daily();
                match daily {
                    Ok(q) => {
                        println!("Daily Quote: {}", q.quote);
                        Ok(())
                    }
                    Err(e) => Err(e.to_string().into()),
                }
            }
            //  TODO: Implement getting a single and all quotes in the database.
            QuoteCommands::Get { id } => {
                if id.is_none() {
                    let quotes = get_quotes_all();
                    match quotes {
                        Ok(q) => {
                            for quote in q {
                                println!("{} - {}", quote.id, quote.quote);
                            }
                            Ok(())
                        }
                        Err(e) => Err(e.to_string().into()),
                    }
                } else {
                    let quote = get_quote_by_id(id.unwrap());
                    match quote {
                        Ok(q) => {
                            println!("{} - {}", q.id, q.quote);
                            Ok(())
                        }
                        Err(e) => Err(e.to_string().into()),
                    }
                }
            }
            QuoteCommands::Random => {
                let quote_res = get_quote_random();

                match quote_res {
                    Ok(q) => {
                        println!("{} - {}", q.id, q.quote);
                        Ok(())
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        Ok(())
                    }
                }
            }
        }
    }
}
