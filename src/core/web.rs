use std::{collections::HashMap, os::unix::process::CommandExt, process::Command};

use reqwest::{Method, Url, header::USER_AGENT};

pub struct SearchParams {
    pub url: String,
    pub args: HashMap<String, String>,
}

impl SearchParams {
    pub fn new(query: &String) -> Self {
        let mut s = Self::default();
        s.args.insert("q".to_string(), query.clone());
        s
    }
}

impl Default for SearchParams {
    fn default() -> Self {
        let mut s = SearchParams {
            url: String::from("https://www.google.com/search"),
            args: HashMap::new(),
        };
        //  Language
        s.args.insert(String::from("hl"), String::from("en"));
        //  Location/Country
        s.args.insert("gl".to_string(), "us".to_string());
        //  Language Restriction
        s.args.insert("lr".to_string(), "lang_en".to_string());
        //  Country Restriction
        s.args.insert("cr".to_string(), "countryUS".to_string());
        //  Query
        // s.args
        //     .insert("q".to_string(), "Spyder337 site:github.com".to_string());
        s
    }
}

fn open_url(url: &str) -> () {
    let _ = open::that_detached(url);
}

pub fn basic_search(options: SearchParams, open: &bool) -> () {
    let url = Url::parse_with_params(&options.url, options.args).unwrap();
    println!("Url: {}", url);
    if *open {
        open_url(&url.to_string());
    } else {
        todo!("Implement displaying search results in the terminal.");
    }
}
