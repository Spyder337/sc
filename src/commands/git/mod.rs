pub mod core;

use core::{clone_repo, create_commit};
use std::{
    fs,
    io::{Error, Write},
    path::{Path, PathBuf},
};

use clap::Subcommand;
use git2::{Repository, Status, StatusOptions};

use super::CommandHandler;
use crate::{commands, expand_sanitized_home, sanitize_path, ENV};

use super::time_now;

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
        /// The repository can be a URL must be a https url.
        repo: String,
        /// Directory to clone the repo to.
        ///
        /// If no directory is provided then the repo will be cloned to
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
    changes: &[String],
) -> crate::Result<()> {
    let repo = Repository::open(".");
    if repo.is_err() {
        return Err(Box::new(repo.err().unwrap()));
    }

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
        //  Equivalent to git add --update
        let res = commands::git::core::add_files(&path_specs, None);

        if res.is_ok() {
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
        let has_changes = !changes.is_empty();

        //  If the changes are not provided then we generate a timestamp for the
        // first line of the commit message.
        if !has_changes {
            main_change = format!("Updated: {}", time_str).to_string();
        } else {
            main_change = changes.first().unwrap().to_string();
            let change_list = changes.to_owned();
            for item in change_list.iter().skip(1) {
                change_msg.push_str(&format!("- {}\n", item));
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

        for s in statuses.unwrap().iter() {
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

        if res.is_ok() {
            println!("Commit was successful.");
        } else {
            return Err(Box::new(res.err().unwrap()));
        }

        Ok(())
    }
}

fn traverse_git_dirs(dir: &Path, root: &PathBuf, paths: &mut Vec<Box<Path>>) -> crate::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            //  If the parent path is the git_dir then traverse again.
            if path.parent().unwrap() == root {
                let _ = traverse_git_dirs(&path, root, paths);
            }
            //  If the grandparent path is the git_dir then add the path to the list.
            else if path.parent().unwrap().parent().unwrap() == root {
                paths.push(path.into_boxed_path());
            }
        }
    }
    Ok(())
}

fn git_list(json: bool) -> crate::Result<()> {
    let env = crate::ENV.lock().unwrap();
    let dir = expand_sanitized_home(env.git_dir.clone().as_path());
    let exists = dir.exists();
    if !exists {
        println!("Directory does not exist.");
        println!("{:?}", dir);
        return Err(Box::new(Error::new(
            std::io::ErrorKind::NotFound,
            "Directory does not exist.",
        )));
    }
    let mut paths: Vec<Box<Path>> = Vec::new();
    let res= traverse_git_dirs(&dir, &dir, &mut paths);

    match res {
        Ok(_) => {
            if !json {
                println!("Listing repos in: {dir:?}");

                println!("Directories:");
                for path in paths {
                    println!("{:?}", sanitize_path(&path));
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
        },
        Err(e) => Err(e),
    }
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

    res.text().unwrap()
}

fn get_ignore_list(name: &Option<String>) -> Vec<String> {
    let url = "https://www.toptal.com/developers/gitignore/api/list?format=lines";
    // println!("Url: {}", url);
    let res = reqwest::blocking::get(url).unwrap();
    // println!("{:#?}", res);
    let body = res.text().unwrap();
    if name.is_none() {
        body.lines().map(|x| x.to_string()).collect()
    } else {
        body.lines()
            .filter(|x| x.contains(name.clone().unwrap().as_str()))
            .map(|x| x.to_string())
            .collect()
    }
}
