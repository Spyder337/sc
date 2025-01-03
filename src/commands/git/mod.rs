mod core;

use core::{clone_repo, create_commit};
use std::{
    fs,
    io::{Error, Write},
    path::Path,
};

use clap::Subcommand;
use git2::{Repository, Status, StatusOptions};

use crate::{ENV, commands, environment::time_now};

use super::CommandHandler;
/// A set of git utilities.
#[derive(Debug, Subcommand)]
pub(crate) enum GitCommands {
    /// Initialize a new git repository.
    New {
        name: String,
        ignores: Option<Vec<String>>,
    },
    /// Clone a repository.
    Clone {
        /// Repository to clone.
        ///
        /// The repository can be a URL or a shorthand like `owner/repo`.
        /// The url can be a ssh or https url.
        repo: String,
        /// Directory to clone the repo to.
        ///
        /// If no repo is provided then the repo will be cloned to
        /// `git_dir/owner/repo`.
        dir: Option<String>,
    },
    /// List cloned repositories in the `git_dir`.
    List {
        /// Return the list in JSON format.
        json: Option<bool>,
    },
    /// Stage files and commit them.
    AddCommit {
        /// File paths to stage.
        #[arg(required = true, num_args(1..), value_delimiter = ',')]
        paths: Vec<String>,
        #[arg(short = 'c', long, required = true, num_args(1..), value_delimiter = ',')]
        /// Changes made in the commit.
        changes: Vec<String>,
    },
    Ignore {
        #[command(subcommand)]
        command: GitIgnoreCommands,
    },
}

impl CommandHandler for GitCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            GitCommands::New { name, ignores } => new_repo(name, ignores.clone()),
            GitCommands::Clone { repo, dir } => clone_repo(repo, dir),
            GitCommands::List { json } => git_list(json.unwrap_or(false)),
            GitCommands::AddCommit { paths, changes } => {
                // TODO: Implement proper error handling.
                let res = add_commit(&Some(paths.clone()), &None, changes);
                if res.is_err() {
                    Err(res.err().unwrap())
                } else {
                    Ok(())
                }
            }
            GitCommands::Ignore { command } => command.handle(),
        }
    }
}

fn new_repo(name: &str, ignores: Option<Vec<String>>) -> crate::Result<()> {
    let author = ENV.lock().unwrap().git_name.clone();
    let path = ENV
        .lock()
        .unwrap()
        .git_dir
        .clone()
        .join(Path::new(&author))
        .join(name);
    println!("Creating new repo: {:?}", path);
    //  Create the directory.
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
    let ignore_path = path.join(".gitignore");
    let ignore_txt = fetch_ignores(ignores.unwrap().as_slice());

    let repo = Repository::init(&path);

    //  Check if the repo was created successfully.
    if let Err(e) = repo {
        println!("Failed to create the repo.\n{:?}", e);
        return Err(Box::new(e));
    }

    //  Get the repo's path from the object.
    let path = repo.unwrap().workdir().unwrap().to_path_buf();

    println!("Repo created at: {:#?}", path);

    let mut ignore_file = fs::File::create(&ignore_path).unwrap();
    let write_res = ignore_file.write(ignore_txt.as_bytes());
    if write_res.is_err() {
        return Err(Box::new(write_res.err().unwrap()));
    }
    Ok(())
}

/// Add and commit files to the repository.
///
/// If paths is None then all files in the repository will be staged.
/// If remove is true then the files will be removed from the staging area.
/// If changes is empty then the commit message will be a timestamp.    
fn add_commit(
    paths: &Option<Vec<String>>,
    remove: &Option<bool>,
    changes: &Vec<String>,
) -> crate::Result<()> {
    let repo = Repository::open(".");
    if repo.is_err() {
        return Err(Box::new(repo.err().unwrap()));
    }

    println!("Paths: {paths:?}");

    let path_specs = paths.clone().unwrap_or(vec![".".to_string()]);

    let r = &mut repo.unwrap();
    let mut _removing = false;
    if let Some(val) = remove {
        _removing = *val;
        if _removing {
            println!("Deleting files from being staged.");
            //  Git Command: git restore --staged $path
        }
        todo!("Implement unstaging files.")
    } else {
        // println!("Staging new files...");

        //  Equivalent to git add --update
        let res = commands::git::core::add_files(&path_specs, None);

        if let Ok(_) = res {
            println!("Files staged successfully.");
        } else {
            return Err(res.err().unwrap());
        }

        //  Git Command: git stage $path
        //  Use `git status -s` to generate an organized change list.
        let mut commit_msg = String::with_capacity(1536);
        let mut status_msg = String::with_capacity(512);
        let mut change_msg = String::with_capacity(512);
        //  If we're staging files then we should generate a commit for the
        //  module too.
        let main_change: String;
        let time = time_now();
        let time_str = time.format("%Y-%m-%d %H:%M:%S");
        let has_changes = changes.len() > 1;

        //  If the changes are not provided then we generate a timestamp for the
        // first line of the commit message.
        if !has_changes {
            main_change = format!("Updated: {}", time_str).to_string();
        } else {
            main_change = changes.first().unwrap().to_string();
            let change_list = changes.clone();
            for i in 1..change_list.len() {
                if change_list[i].is_empty() {
                    continue;
                }
                change_msg.push_str(&format!("- {}\n", change_list[i]));
            }
        }

        //  Generate the first line.
        commit_msg.push_str(&main_change);
        commit_msg.push_str("\n\n");

        //  If there are changes then append them.
        if has_changes {
            commit_msg.push_str(format!("Updated: {}\n", time_str).as_str());
            if !change_msg.is_empty() {
                commit_msg.push_str("\nChanges:\n");
                commit_msg.push_str(&change_msg);
            }
        }

        // Generate the status message
        let statuses = r.statuses(Some(&mut StatusOptions::new()));

        for s in statuses.unwrap().iter().map(|a| a) {
            match s.status() {
                Status::INDEX_NEW => {
                    status_msg.push_str(format!("A {}\n", s.path().unwrap()).as_str());
                }
                Status::INDEX_MODIFIED => {
                    status_msg.push_str(format!("M {}\n", s.path().unwrap()).as_str());
                }
                Status::INDEX_DELETED => {
                    status_msg.push_str(format!("D {}\n", s.path().unwrap()).as_str());
                }
                _ => (),
            }
        }

        //  Append the status message.
        commit_msg.push_str("\nFiles Changed:\n");
        commit_msg.push_str(&status_msg);
        println!("Commit Message Generated: \n\n{}", commit_msg);

        let res = create_commit(r, commit_msg);

        if let Ok(_) = res {
            println!("Commit was successful.");
        } else {
            return Err(Box::new(res.err().unwrap()));
        }

        Ok(())
    }
}

fn traverse_dir(dir: Box<Path>, acc: &mut Vec<Box<Path>>) {
    let entries = fs::read_dir(dir).unwrap();
    let git_dir = ENV.lock().unwrap().git_dir.clone();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if path.components().into_iter().any(|x| {
                x.as_os_str() == "target"
                    || x.as_os_str() == "obj"
                    || x.as_os_str() == ".git"
                    || x.as_os_str() == "bin"
            }) {
                continue;
            }

            if path.parent().unwrap().parent().unwrap() == git_dir.to_path_buf() {
                acc.push(entry.path().into_boxed_path());
            }

            traverse_dir(path.into_boxed_path(), acc);
        }
    }
}

fn git_list(json: bool) -> crate::Result<()> {
    let dir = crate::ENV.lock().unwrap().git_dir.clone();
    let exists = dir.exists();
    if !exists {
        println!("Directory does not exist.");
        return Err(Box::new(Error::new(
            std::io::ErrorKind::NotFound,
            "Directory does not exist.",
        )));
    }
    let mut paths: Vec<Box<Path>> = Vec::new();
    traverse_dir(dir.clone().into_boxed_path(), &mut paths);
    if !json {
        println!("Listing repos in: {dir:?}");

        println!("Directories:");
        for path in paths {
            println!("{:?}", path);
        }
    } else {
        let json = serde_json::to_string_pretty(&paths);
        if json.is_err() {
            return Err(Box::new(json.err().unwrap()));
        } else {
            println!("{}", json.unwrap());
        }
    }
    Ok(())
}

/// A set of .gitignore utilities.
#[derive(Debug, Subcommand)]
pub(crate) enum GitIgnoreCommands {
    /// List the available .gitignore templates.
    List {
        /// Filter the list of templates.
        name: Option<String>,
    },
    /// Generate a new .gitignore file using the templates provided.
    ///
    /// If the `create_file` flag is set then the file will be created in the
    /// current directory. Otherwise, the output will be printed to the console.
    Fetch {
        /// Templates to use in the .gitignore file.
        templates: Vec<String>,
        /// Create a .gitignore file.
        #[arg(long)]
        create_file: bool,
    },
}

impl CommandHandler for GitIgnoreCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            GitIgnoreCommands::List { name } => {
                let ignore_list = get_ignore_list(name);
                for ignore in ignore_list {
                    println!("{}", ignore);
                }
                Ok(())
            }
            GitIgnoreCommands::Fetch {
                templates,
                create_file,
            } => {
                let ignore_txt = fetch_ignores(templates);
                if *create_file {
                    let mut ignore_file = fs::File::create(".gitignore").unwrap();
                    let write_res = ignore_file.write(ignore_txt.as_bytes());
                    if write_res.is_err() {
                        return Err(Box::new(write_res.err().unwrap()));
                    }
                } else {
                    println!("{}", ignore_txt);
                }
                Ok(())
            }
        }
    }
}

fn fetch_ignores(ignores: &[String]) -> String {
    if ignores.is_empty() {
        println!("No ignore files provided.");
        return String::new();
    }
    let url = "https://www.toptal.com/developers/gitignore/api/";
    let full_url = format!("{}{}", url, ignores.join(","));
    let res = reqwest::blocking::get(full_url).unwrap();
    let body = res.text().unwrap();
    body
}

fn get_ignore_list(name: &Option<String>) -> Vec<String> {
    let url = "https://www.toptal.com/developers/gitignore/api/list?format=lines";
    // println!("Url: {}", url);
    let res = reqwest::blocking::get(url).unwrap();
    // println!("{:#?}", res);
    let body = res.text().unwrap();
    if name.is_none() {
        return body.lines().map(|x| x.to_string()).collect();
    } else {
        return body
            .lines()
            .filter(|x| x.contains(name.clone().unwrap().as_str()))
            .map(|x| x.to_string())
            .collect();
    }
}
