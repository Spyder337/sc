use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// Environment settings for the application.
#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    /// Git user name to use in commit signatures.
    pub git_name: String,
    /// Git email to use in commit signatures.
    pub git_email: String,
    /// Git directory to clone repos to.
    pub git_dir: PathBuf,
    /// Git ignore url base.
    pub git_ignore_url: String,
    /// Database connection string.
    pub conn_str: String,
    /// API Key for [Custom Search JSON API](https://developers.google.com/custom-search/v1/overview).
    pub google_search_api_key: String,
    /// Engine ID for [Custom Search JSON API](https://developers.google.com/custom-search/v1/using_rest#making_a_request).
    pub google_search_engine_id: String,
}

impl Environment {
    /// Saves the environment settings.
    ///
    /// The old settings are overwritten.
    pub fn save(&self) -> () {
        self.create_file();
    }

    /// Loads the environment settings.
    ///
    /// If the config file is not found, the default settings are returned.
    pub fn load() -> Self {
        let env = Environment::default();
        //  Try to load the config file.
        let file_res = File::open(crate::CONFIG_FILE.clone());

        match file_res {
            Ok(mut file) => {
                let mut buf = String::new();
                let res = file.read_to_string(&mut buf);

                match res {
                    Ok(_) => {
                        let env: Environment = Self::deserialize(&buf);
                        env
                    }
                    Err(_) => {
                        env.create_file();
                        env
                    }
                }
            }
            Err(_) => {
                env.create_file();
                env
            }
        }
    }

    /// Serializes the environment settings to a TOML string.
    pub fn serialize(&self) -> String {
        toml::to_string(&self).unwrap()
    }

    pub fn deserialize(toml: &str) -> Self {
        toml::from_str(toml).unwrap()
    }

    /// Creates the config file.
    ///
    /// If the config file already exists, it is overwritten.
    fn create_file(&self) -> () {
        if !crate::APP_DIR.exists() {
            std::fs::create_dir_all(crate::APP_DIR.clone()).unwrap();
        }
        let mut file = File::create(crate::CONFIG_FILE.clone()).unwrap();
        let toml = self.serialize();
        file.write_all(toml.as_bytes()).unwrap();
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            git_name: "User".into(),
            git_email: "user.name@email.com".into(),
            git_dir: "~/Code".into(),
            git_ignore_url: "https://www.toptal.com/developers/gitignore/api/".into(),
            conn_str: crate::SQL_FILE.display().to_string(),
            google_search_api_key: "google_search_api_key".into(),
            google_search_engine_id: "google_search_engine_id".into(),
        }
    }
}

pub fn time_now() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}
