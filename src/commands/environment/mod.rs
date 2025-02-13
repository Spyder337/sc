pub mod core;
use super::Environment;
use crate::{Colorize, ENV};
use clap::Subcommand;

use super::CommandHandler;
/// A set of utilities for interacting with the environment.
#[derive(Debug, Subcommand)]
pub(crate) enum EnvCommands {
    /// Set an environment variable.
    ///
    /// The environment variables are set using flags. If no flags are
    /// provided, the command will exit.
    Set {
        /// Git user name to use in commit signatures.
        #[arg(short = None, long)]
        git_name: Option<String>,
        /// Git email to use in commit signatures.
        #[arg(short = None, long)]
        git_email: Option<String>,
        /// Git directory to clone repos to.
        #[arg(short = None, long)]
        git_dir: Option<String>,
        /// Git ignore url base.
        #[arg(short = None, long)]
        git_ignore_url: Option<String>,
        /// Database connection string.
        #[arg(short = None, long)]
        conn_str: Option<String>,
        #[arg(short = None, long)]
        google_search_api_key: Option<String>,
        #[arg(short = None, long)]
        google_search_engine_id: Option<String>,
    },
    /// Get an environment variable.
    ///
    /// If no flags are provided, all environment variables are returned.
    Get {
        /// Git user name to use in commit signatures.
        #[arg(short = None, long)]
        git_name: bool,
        /// Git email to use in commit signatures.
        #[arg(short = None, long)]
        git_email: bool,
        /// Git directory to clone repos to.
        #[arg(short = None, long)]
        git_dir: bool,
        /// Git ignore url base.
        #[arg(short = None, long)]
        git_ignore_url: bool,
        /// Database connection string.
        #[arg(short = None, long)]
        conn_str: bool,
        #[arg(short = None, long)]
        google_search_api_key: bool,
        #[arg(short = None, long)]
        google_search_engine_id: bool,
    },
    /// Reset an environment variable.
    ///
    /// If no flags are provided, all environment variables are reset.
    Reset {
        /// Git user name to use in commit signatures.
        #[arg(short = None, long)]
        git_name: bool,
        /// Git email to use in commit signatures.
        #[arg(short = None, long)]
        git_email: bool,
        /// Git directory to clone repos to.
        #[arg(short = None, long)]
        git_dir: bool,
        /// Git ignore url base.
        #[arg(short = None, long)]
        git_ignore_url: bool,
        /// Database connection string.
        #[arg(short = None, long)]
        conn_str: bool,
        #[arg(short = None, long)]
        google_search_api_key: bool,
        #[arg(short = None, long)]
        google_search_engine_id: bool,
    },
    /// Save the environment settings.
    Save,
    /// Load the environment settings.
    Load,
    /// Print the program's file locations.
    Files,
    /// Generate a .env file.
    GenerateDotEnv,
}

impl CommandHandler for EnvCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            EnvCommands::Set {
                git_name,
                git_email,
                git_dir,
                git_ignore_url,
                conn_str,
                google_search_api_key,
                google_search_engine_id,
            } => set_env(
                git_name,
                git_email,
                git_dir,
                git_ignore_url,
                conn_str,
                google_search_api_key,
                google_search_engine_id,
            ),
            EnvCommands::Get {
                git_name,
                git_email,
                git_dir,
                git_ignore_url,
                conn_str,
                google_search_api_key,
                google_search_engine_id,
            } => get_env(
                git_name,
                git_email,
                git_dir,
                git_ignore_url,
                conn_str,
                google_search_api_key,
                google_search_engine_id,
            ),
            EnvCommands::Reset {
                git_name,
                git_email,
                git_dir,
                git_ignore_url,
                conn_str,
                google_search_api_key,
                google_search_engine_id,
            } => reset_env(
                git_name,
                git_email,
                git_dir,
                git_ignore_url,
                conn_str,
                google_search_api_key,
                google_search_engine_id,
            ),
            EnvCommands::Save => {
                crate::ENV.lock().unwrap().save();
                Ok(())
            }
            EnvCommands::Load => {
                *crate::ENV.lock().unwrap() = Environment::load();
                Ok(())
            }
            EnvCommands::Files => get_files(),
            EnvCommands::GenerateDotEnv => {
                let _ = crate::database::generate_dotenv();
                Ok(())
            }
            // EnvCommands::InitDatabase => {
            //     let _ = crate::database::init_database();
            //     Ok(())
            // }
        }
    }
}

fn set_env(
    git_name: &Option<String>,
    git_email: &Option<String>,
    git_dir: &Option<String>,
    git_ignore_url: &Option<String>,
    conn_str: &Option<String>,
    google_search_api_key: &Option<String>,
    google_search_engine_id: &Option<String>,
) -> crate::Result<()> {
    let env = &mut ENV.lock().unwrap();
    if let Some(git_name) = git_name {
        println!("Setting {} to: {}", "Git User Name".magenta(), git_name);
        env.git_name = git_name.clone();
        println!("{} set to: {}", "Git User Name".magenta(), env.git_name);
    }
    if let Some(git_email) = git_email {
        println!("Setting {} to: {}", "Git Email".magenta(), git_email);
        env.git_email = git_email.clone();
        println!("{} set to: {}", "Git Email".magenta(), env.git_email);
    }
    if let Some(git_dir) = git_dir {
        println!("Setting {} to: {}", "Git Directory".magenta(), git_dir);
        env.git_dir = git_dir.clone().into();
        println!(
            "{} set to: {}",
            "Git Directory".magenta(),
            env.git_dir.display()
        );
    }
    if let Some(git_ignore_url) = git_ignore_url {
        println!(
            "Setting {} to: {}",
            "Git Ignore URL".magenta(),
            git_ignore_url
        );
        env.git_ignore_url = git_ignore_url.clone();
        println!(
            "{} set to: {}",
            "Git Ignore URL".magenta(),
            env.git_ignore_url
        );
    }
    if let Some(conn_str) = conn_str {
        println!("Setting {} to: {}", "Connection String".magenta(), conn_str);
        env.conn_str = conn_str.clone();
        println!("{} set to: {}", "Connection String".magenta(), env.conn_str);
    }
    if let Some(google_search_api_key) = google_search_api_key {
        println!(
            "Setting {} to: {}",
            "Google Search API Key".magenta(),
            google_search_api_key
        );
        env.google_search_api_key = google_search_api_key.clone();
        println!(
            "{} set to: {}",
            "Google Search API Key".magenta(),
            env.google_search_engine_id
        );
    }
    if let Some(google_search_engine_id) = google_search_engine_id {
        println!(
            "Setting {} to: {}",
            "Google Search Engine ID".magenta(),
            google_search_engine_id
        );
        env.google_search_engine_id = google_search_engine_id.clone();
        println!(
            "{} set to: {}",
            "Google Search Engine ID".magenta(),
            env.google_search_engine_id
        );
    }
    env.save();
    Ok(())
}

fn get_env(
    git_name: &bool,
    git_email: &bool,
    git_dir: &bool,
    git_ignore_url: &bool,
    conn_str: &bool,
    google_search_api_key: &bool,
    google_search_engine_id: &bool,
) -> crate::Result<()> {
    let mut add_all = false;
    if !git_name
        && !git_email
        && !git_dir
        && !git_ignore_url
        && !conn_str
        && !google_search_api_key
        && !google_search_engine_id
    {
        add_all = true;
    }
    let mut env_str = String::with_capacity(256);
    let env = ENV.lock().unwrap();
    if add_all || *git_name {
        env_str.push_str(&format!(
            "{}: {}\n",
            "Git User Name".magenta(),
            env.git_name
        ));
    }
    if add_all || *git_email {
        env_str.push_str(&format!("{}: {}\n", "Git Email".magenta(), env.git_email));
    }
    if add_all || *git_dir {
        env_str.push_str(&format!(
            "{}: {}\n",
            "Git Directory".magenta(),
            env.git_dir.display()
        ));
    }
    if add_all || *git_ignore_url {
        env_str.push_str(&format!(
            "{}: {}\n",
            "Git Ignore URL".magenta(),
            env.git_ignore_url
        ));
    }
    if add_all || *conn_str {
        env_str.push_str(&format!(
            "{}: {}\n",
            "Connection String".magenta(),
            env.conn_str
        ));
    }
    if add_all || *google_search_api_key {
        env_str.push_str(&format!(
            "{}: {}\n",
            "Google Search API Key".magenta(),
            env.google_search_api_key
        ));
    }
    if add_all || *google_search_engine_id {
        env_str.push_str(&format!(
            "{}: {}\n",
            "Google Search Engine ID".magenta(),
            env.google_search_engine_id
        ));
    }
    print!("{}", env_str);
    Ok(())
}

fn reset_env(
    git_name: &bool,
    git_email: &bool,
    git_dir: &bool,
    git_ignore_url: &bool,
    conn_str: &bool,
    google_search_api_key: &bool,
    google_search_engine_id: &bool,
) -> crate::Result<()> {
    let default_env = Environment::default();
    let env = &mut ENV.lock().unwrap();
    if *git_name {
        env.git_name = default_env.git_name;
    }
    if *git_email {
        env.git_email = default_env.git_email;
    }
    if *git_dir {
        env.git_dir = default_env.git_dir;
    }
    if *git_ignore_url {
        env.git_ignore_url = default_env.git_ignore_url;
    }
    if *conn_str {
        env.conn_str = default_env.conn_str;
    }
    if *google_search_api_key {
        env.google_search_api_key = default_env.google_search_api_key;
    }
    if *google_search_engine_id {
        env.google_search_engine_id = default_env.google_search_engine_id;
    }
    env.save();
    Ok(())
}

fn get_files() -> crate::Result<()> {
    println!(
        "\n{}: {}",
        "Config Directory".magenta_bright(),
        crate::APP_DIR.to_str().unwrap().green_bright()
    );
    println!(
        "{}: {}",
        "Config File".magenta_bright(),
        crate::CONFIG_FILE.to_str().unwrap().green_bright()
    );
    println!(
        "{}: {}\n",
        "Database File".magenta_bright(),
        crate::SQL_FILE.to_str().unwrap().green_bright()
    );
    Ok(())
}
