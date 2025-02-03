use std::collections::HashMap;

use reqwest::Url;

use crate::ENV;

/// Search parameters for a google web search.
#[derive(Debug)]
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

    /// Creates a new set of search parameters that is compatible with the
    /// Google Custom Search JSON API.
    pub fn new_json(query: &String) -> Self {
        let mut s = Self::default();
        s.args.insert("q".to_string(), query.clone());
        s.add_api_data();
        s
    }

    pub fn add_api_data(&mut self) {
        self.args.insert(
            "key".to_string(),
            ENV.lock().unwrap().google_search_api_key.clone(),
        );
        self.args.insert(
            "cx".to_string(),
            ENV.lock().unwrap().google_search_engine_id.clone(),
        );
    }
}

impl From<SearchParams> for Url {
    fn from(search: SearchParams) -> Self {
        Url::parse_with_params(&search.url, search.args).unwrap()
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

/// Open a URL in the default browser.
fn open_url(url: &str) -> crate::Result<()> {
    let res = open::that_detached(url);

    if res.is_ok() {
        println!("Opened URL: {}", url);
        Ok(())
    } else {
        Err(Box::new(res.err().unwrap()))
    }
}

/// Perform a basic search using the given search parameters.
///
/// Optionally open the search results in the default browser.
pub fn basic_search(mut options: SearchParams, json: &bool) -> crate::Result<()> {
    options.add_api_data();
    let url: Url = options.into();
    // println!("Url: {}", url);
    if *json {
        println!("Fetching web page: {}", url);
        basic_search_json(url)
    } else {
        println!("Opening URL: {}", url);
        basic_search_open(url)
    }
}

fn basic_search_open(url: Url) -> crate::Result<()> {
    let res = open_url(url.as_ref());
    res
}

fn basic_search_json(url: Url) -> crate::Result<()> {
    let req = reqwest::blocking::get(url.to_string())?;
    let body = req.text()?;
    println!("{}", body);
    Ok(())
}

pub fn query_string_builder(
    query: &str,
    site: &Option<String>,
    allintext: &Option<String>,
) -> String {
    let mut query_string = query.to_string();
    if let Some(site) = site {
        query_string.push_str(&format!(" site:{}", site));
    }
    if let Some(all_in_text) = allintext {
        query_string.push_str(&format!(" allintext:{}", all_in_text));
    }
    query_string
}
