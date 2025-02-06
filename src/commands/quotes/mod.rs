pub mod core;

use clap::{Subcommand, arg};

use core::{add_quote, get_daily, get_quote_by_id, get_quotes_all};
use std::io::stdin;

use crate::database::sqlite::get_quote_random;

use super::CommandHandler;

//  TODO: Implement the commands for calling the functions.

/// Add and get quotes from the database.
#[derive(Debug, Subcommand)]
pub(crate) enum QuoteCommands {
    /// Add a new quote to the database.
    /// 
    /// If the author or quote is not provided, the user will be prompted to enter them.
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
            QuoteCommands::Add { author, quote } => {
                if author.is_none() && quote.is_none() {
                    let mut quote_input = String::new();
                    let mut author_input = String::new();
                    println!("Enter the quote: \nPress Enter to submit.");
                    while quote_input.is_empty() {
                        stdin().read_line(&mut quote_input).unwrap();
                    }
                    println!("Enter the author: \nPress Enter to submit.");
                    while author_input.is_empty() {
                        stdin().read_line(&mut author_input).unwrap();
                    }
                    let res = add_quote(quote_input.trim(), author_input.trim());   
                    match res {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.to_string().into()),
                        
                    }
                } else if author.is_none() || quote.is_none() {
                    if author.is_none() {
                        let mut author_input = String::new();
                        println!("Enter the author: \nPress Enter to submit.");
                        while author_input.is_empty() {
                            stdin().read_line(&mut author_input).unwrap();
                        }
                        let res = add_quote(quote.as_ref().unwrap(), author_input.trim());
                        match res {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e.to_string().into()),
                        }
                    } else {
                        let mut quote_input = String::new();
                        println!("Enter the quote: \nPress Enter to submit.");
                        while quote_input.is_empty() {
                            stdin().read_line(&mut quote_input).unwrap();
                        }
                        let res = add_quote(quote_input.trim(), author.as_ref().unwrap());
                        match res {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e.to_string().into()),
                        }
                    }
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
                        println!("{}", q);
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
                                println!("{}", quote);
                            }
                            Ok(())
                        }
                        Err(e) => Err(e.to_string().into()),
                    }
                } else {
                    let quote = get_quote_by_id(id.unwrap());
                    match quote {
                        Ok(q) => {
                            println!("{}", q);
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
                        println!("{}", q);
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
