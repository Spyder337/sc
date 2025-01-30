use clap::Subcommand;

use crate::{ENV, environment::Environment};

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
    },
    /// Save the environment settings.
    Save,
    /// Load the environment settings.
    Load,
    /// Print the program's file locations.
    Files,
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
            } => set_env(git_name, git_email, git_dir, git_ignore_url, conn_str),
            EnvCommands::Get {
                git_name,
                git_email,
                git_dir,
                git_ignore_url,
                conn_str,
            } => get_env(git_name, git_email, git_dir, git_ignore_url, conn_str),
            EnvCommands::Reset {
                git_name,
                git_email,
                git_dir,
                git_ignore_url,
                conn_str,
            } => reset_env(git_name, git_email, git_dir, git_ignore_url, conn_str),
            EnvCommands::Save => {
                crate::ENV.lock().unwrap().save();
                Ok(())
            }
            EnvCommands::Load => {
                *crate::ENV.lock().unwrap() = Environment::load();
                Ok(())
            }
            EnvCommands::Files => get_files(),
        }
    }
}

fn set_env(
    git_name: &Option<String>,
    git_email: &Option<String>,
    git_dir: &Option<String>,
    git_ignore_url: &Option<String>,
    conn_str: &Option<String>,
) -> crate::Result<()> {
    if let Some(git_name) = git_name {
        println!("Setting git name to: {}", git_name);
        ENV.lock().unwrap().git_name = git_name.clone();
        println!("Git name set to: {}", ENV.lock().unwrap().git_name);
    }
    if let Some(git_email) = git_email {
        println!("Setting git email to: {}", git_email);
        ENV.lock().unwrap().git_email = git_email.clone();
        println!("Git email set to: {}", ENV.lock().unwrap().git_email);
    }
    if let Some(git_dir) = git_dir {
        println!("Setting git directory to: {}", git_dir);
        ENV.lock().unwrap().git_dir = git_dir.clone().into();
        println!("Git directory set to: {}", ENV.lock().unwrap().git_dir.display());
    }
    if let Some(git_ignore_url) = git_ignore_url {
        println!("Setting git ignore url to: {}", git_ignore_url);
        ENV.lock().unwrap().git_ignore_url = git_ignore_url.clone();
        println!("Git ignore url set to: {}", ENV.lock().unwrap().git_ignore_url);
    }
    if let Some(conn_str) = conn_str {
        println!("Setting connection string to: {}", conn_str);
        ENV.lock().unwrap().conn_str = conn_str.clone();
        println!("Connection string set to: {}", ENV.lock().unwrap().conn_str);
    }
    ENV.lock().unwrap().save();
    Ok(())
}

fn get_env(
    git_name: &bool,
    git_email: &bool,
    git_dir: &bool,
    git_ignore_url: &bool,
    conn_str: &bool,
) -> crate::Result<()> {
    let mut add_all = false;
    if !git_name && !git_email && !git_dir && !git_ignore_url && !conn_str {
        add_all = true;
    }
    let mut env_str = String::with_capacity(256);
    if add_all || *git_name {
        env_str.push_str(&format!("Git Name: {}\n", ENV.lock().unwrap().git_name));
    }
    if add_all || *git_email {
        env_str.push_str(&format!("Git Email: {}\n", ENV.lock().unwrap().git_email));
    }
    if add_all || *git_dir {
        env_str.push_str(&format!(
            "Git Directory: {}\n",
            ENV.lock().unwrap().git_dir.display()
        ));
    }
    if add_all || *git_ignore_url {
        env_str.push_str(&format!(
            "Git Ignore URL: {}\n",
            ENV.lock().unwrap().git_ignore_url
        ));
    }
    if add_all || *conn_str {
        env_str.push_str(&format!(
            "Connection String: {}\n",
            ENV.lock().unwrap().conn_str
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
) -> crate::Result<()> {
    let env = Environment::default();
    if *git_name {
        ENV.lock().unwrap().git_name = env.git_name;
    }
    if *git_email {
        ENV.lock().unwrap().git_email = env.git_email;
    }
    if *git_dir {
        ENV.lock().unwrap().git_dir = env.git_dir;
    }
    if *git_ignore_url {
        ENV.lock().unwrap().git_ignore_url = env.git_ignore_url;
    }
    if *conn_str {
        ENV.lock().unwrap().conn_str = env.conn_str;
    }
    ENV.lock().unwrap().save();
    Ok(())
}

fn get_files() -> crate::Result<()> {
    println!("Config Directory: {}", crate::APP_DIR.display());
    println!("Config File: {}", crate::CONFIG_FILE.display());
    println!("Database File: {}", crate::SQL_FILE.display());
    Ok(())
}
