use std::path::Path; // Add this line to import the crate

use clap::{Subcommand, ValueHint, command};
use git2::Repository;

use crate::time_now;
#[derive(Subcommand, Debug)]
pub enum GitCommands {
    #[command(
        about = "Stage changes and commit them.",
        long_about = r###"Add changes in the current directory or an optional one.
Then generate a change list or take one from the user.
Generate a generic commit message header with timestamps and then combine
that with the optional change list.

If a path is not provided then '.' is used as the path.

If the remove flag is provided then the files matching the path are unstaged."###
    )]
    Update {
        #[arg(short = 'p', long)]
        paths: Option<Vec<String>>,
        #[arg(short = 'r', long)]
        remove: Option<bool>,
        #[arg(short = 'c', long, value_delimiter = ',', num_args = 0..)]
        changes: Option<Vec<String>>,
    },
    #[command(
        about = "List repos in the user git directory.",
        long_about = "List the user repos and the pulled returned as a json object."
    )]
    List {},
    #[command(about = "Set the repo directory.")]
    SetDir {
        #[arg(value_hint = ValueHint::DirPath)]
        path: String,
    },
    #[command(about = "Get the repo directory.")]
    GetDir {},
    #[command(
        about = "Fetch a .gitignore file.",
        long_about = r#"Fetches a .gitignore file from https://github.com/github/gitignore
Common Ignores: 
    - Rust
    - VisualSudio
    
If multiple ignore files are provided then they are appended to the same file.

To get a list of all of the valid ignore names use the fetch-ignores command."#
    )]
    FetchIgnore {
        #[arg(help = "Ignore file names", short = 'f', long, num_args= (1..),)]
        files: Vec<String>,
    },
    #[command(
        about = "Display valid ignore files.",
        long_about = r###"Fetches ignore files from https://github.com/github/gitignore"###
    )]
    FetchIgnores {},
}

pub(crate) fn handle_commands(command: &GitCommands) {
    match command {
        GitCommands::Update {
            paths,
            remove,
            changes,
        } => update_exec(paths, remove, changes),
        GitCommands::List {} => list_exec(),
        GitCommands::SetDir { path } => set_dir_exec(),
        GitCommands::GetDir {} => {
            let get_dir_exec = get_dir_exec();
            println!("Path: {}", get_dir_exec.to_str().unwrap());
        }
        GitCommands::FetchIgnore { files } => todo!(),
        GitCommands::FetchIgnores {} => {
            let _ = fetch_ignores_exec();
        }
    }
}

/// Add files to the git repository.
/// paths: Path specs.
fn add_files(repo: &mut Repository, paths: &Vec<String>) {
    let mut index = repo.index().unwrap();
    let cb = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
        let status = repo.status_file(path).unwrap();
        let ret = if status.contains(git2::Status::WT_MODIFIED)
            || status.contains(git2::Status::WT_NEW)
        {
            println!("Add '{}'", path.display());
            0
        } else {
            1
        };
        ret
    };

    let cb = Some(cb as &mut git2::IndexMatchedPath);

    index.update_all(paths, cb).unwrap();
    index.write().unwrap();
}

///  # Example Message
///  ```shell
/// $ Update path_to_stage to_remove "Main Change", "Change1", ...
///
/// <Main Change>
///
/// <Time Stamp>
///
/// Changes:
/// - Change 1
/// - Change 2
/// - Change 3
///
/// <Files Changed>
/// ```
fn update_exec(
    paths: &Option<Vec<String>>,
    remove: &Option<bool>,
    changes: &Option<Vec<String>>,
) -> () {
    let mut repo = Repository::open(".");
    if repo.is_err() {
        println!("Was not able to open the repo.\n{:?}", repo.err());
        return;
    }

    let r = &mut repo.unwrap();
    let mut removing = false;
    if let Some(val) = remove {
        removing = *val;
        if removing {
            println!("Deleting files from being staged.");
            //  Git Command: git restore --staged $path
        }
        todo!("Implement unstaging files.")
    } else {
        println!("Staging new files...");

        //  Equivalent to git add --update

        add_files(r, &paths.clone().unwrap());
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

        //  If the changes are not provided then we generate a timestamp for the
        // first line of the commit message.
        if changes.is_none() {
            main_change = format!("Updated: {}", time_str).to_string();
        } else {
            main_change = changes.as_ref().unwrap().first().unwrap().to_string();
            let change_list = changes.as_ref().unwrap();
            if change_list.len() > 1 {
                for i in 1..change_list.len() {
                    if change_list[i].is_empty() {
                        continue;
                    }
                    change_msg.push_str(&format!("- {}\n", change_list[i]));
                }
            }
        }

        //  Generate the first line.
        commit_msg.push_str(&main_change);
        commit_msg.push_str("\n\n");

        //  If there are changes then append them.
        if changes.is_some() {
            commit_msg.push_str(format!("Updated: {}\n", time_str).as_str());
            commit_msg.push_str("Changes:\n");
            commit_msg.push_str(&change_msg);
        }

        //  Append the status message.
        commit_msg.push_str("Files Changed:\n");
        commit_msg.push_str(&status_msg);
        println!("Commit Message Generated: \n{}", commit_msg);
    }
}

fn list_exec() -> () {}
fn set_dir_exec() -> () {}
fn get_dir_exec() -> Box<Path> {
    todo!("Return the git directory from the sqlite database.")
}
fn fetch_ignore_exec(ignores: &[String]) -> String {
    todo!("Implement a fetch ignore function that returns the ignore text.");
    todo!("Implement combining different ignore files.")
}
fn fetch_ignores_exec() -> Vec<String> {
    todo!("Implement fetching a list of valid ignore files.")
}

fn cheat_sheet_exec() -> () {}
