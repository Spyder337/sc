#![allow(dead_code)]
use core::str;
use std::cell::RefCell;
use std::io::Write;
use std::path::Path;
use std::{io, path::PathBuf};

use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{
    Error, ErrorCode, FetchOptions, Oid, Progress, RemoteCallbacks, Repository, Signature,
    SubmoduleIgnore,
};

use crate::expand_sanitized_home;

//
//
//  Code Sourced from https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs.
//
//
#[derive(Eq, PartialEq)]
enum Format {
    Long,
    Short,
    Porcelain,
}

fn show_branch(repo: &Repository, format: &Format) -> Result<(), Error> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return Err(e),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());

    if format == &Format::Long {
        println!(
            "# On branch {}",
            head.unwrap_or("Not currently on any branch")
        );
    } else {
        println!("## {}", head.unwrap_or("HEAD (no branch)"));
    }
    Ok(())
}

// This version of the output prefixes each path with two status columns and
// shows submodule status information.
pub fn message_short(repo: &Repository, statuses: &git2::Statuses) -> String {
    let mut msg: String = String::with_capacity(256);
    for entry in statuses
        .iter()
        .filter(|e| e.status() != git2::Status::CURRENT)
    {
        let mut istatus = match entry.status() {
            s if s.contains(git2::Status::INDEX_NEW) => 'A',
            s if s.contains(git2::Status::INDEX_MODIFIED) => 'M',
            s if s.contains(git2::Status::INDEX_DELETED) => 'D',
            s if s.contains(git2::Status::INDEX_RENAMED) => 'R',
            s if s.contains(git2::Status::INDEX_TYPECHANGE) => 'T',
            _ => ' ',
        };
        let mut wstatus = match entry.status() {
            s if s.contains(git2::Status::WT_NEW) => {
                if istatus == ' ' {
                    istatus = '?';
                }
                '?'
            }
            s if s.contains(git2::Status::WT_MODIFIED) => 'M',
            s if s.contains(git2::Status::WT_DELETED) => 'D',
            s if s.contains(git2::Status::WT_RENAMED) => 'R',
            s if s.contains(git2::Status::WT_TYPECHANGE) => 'T',
            _ => ' ',
        };

        if entry.status().contains(git2::Status::IGNORED) {
            istatus = '!';
            wstatus = '!';
        }
        if istatus == '?' && wstatus == '?' {
            continue;
        }
        let mut extra = "";

        // A commit in a tree is how submodules are stored, so let's go take a
        // look at its status.
        //
        // TODO: check for GIT_FILEMODE_COMMIT
        let status = entry.index_to_workdir().and_then(|diff| {
            let ignore = SubmoduleIgnore::Unspecified;
            diff.new_file()
                .path_bytes()
                .and_then(|s| str::from_utf8(s).ok())
                .and_then(|name| repo.submodule_status(name, ignore).ok())
        });
        if let Some(status) = status {
            if status.contains(git2::SubmoduleStatus::WD_MODIFIED) {
                extra = " (new commits)";
            } else if status.contains(git2::SubmoduleStatus::WD_INDEX_MODIFIED)
                || status.contains(git2::SubmoduleStatus::WD_WD_MODIFIED)
            {
                extra = " (modified content)";
            } else if status.contains(git2::SubmoduleStatus::WD_UNTRACKED) {
                extra = " (untracked content)";
            }
        }

        let (mut a, mut b, mut c) = (None, None, None);
        if let Some(diff) = entry.head_to_index() {
            a = diff.old_file().path();
            b = diff.new_file().path();
        }
        if let Some(diff) = entry.index_to_workdir() {
            a = a.or_else(|| diff.old_file().path());
            b = b.or_else(|| diff.old_file().path());
            c = diff.new_file().path();
        }

        let m = match (istatus, wstatus) {
            ('R', 'R') => format!(
                "RR {} {} {}{}",
                a.unwrap().display(),
                b.unwrap().display(),
                c.unwrap().display(),
                extra
            ),
            ('R', w) => {
                format!(
                    "R{} {} {}{}",
                    w,
                    a.unwrap().display(),
                    b.unwrap().display(),
                    extra
                )
            }
            (i, 'R') => {
                format!(
                    "{}R {} {}{}",
                    i,
                    a.unwrap().display(),
                    c.unwrap().display(),
                    extra
                )
            }
            (i, w) => format!("{}{} {}{}", i, w, a.unwrap().display(), extra),
        };

        msg.push_str(&format!("{}\n", m));
    }

    for entry in statuses
        .iter()
        .filter(|e| e.status() == git2::Status::WT_NEW)
    {
        msg.push_str(&format!(
            "?? {}",
            entry
                .index_to_workdir()
                .unwrap()
                .old_file()
                .path()
                .unwrap()
                .display()
        ));
    }
    msg
}

/// Encapsulates the progress of a clone operation.
struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

/// Print the progress of a clone operation.
fn print(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!();
            state.newline = true;
        }
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else {
        print!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        )
    }
    io::stdout().flush().unwrap();
}

/// Clone a repository.
pub fn clone_repo(url: &str, dir: &Option<String>) -> crate::Result<()> {
    let env = crate::ENV.lock().unwrap();
    
    //  Break the url into multiple components.
    let path: PathBuf;
    let url_components = url.split("/").collect::<Vec<&str>>();
    //  Get the repository name and the user name.
    let mut repo_name: String = url_components.last().unwrap().to_string();
    let user_name = url_components.iter().rev().nth(1).unwrap();
    repo_name = repo_name.replace(".git", "");

    //  Create the path to the repository.
    if let Some(d) = dir {
        path = PathBuf::from(d).join(repo_name);
    } else {
        let temp = env.git_dir.join(user_name).join(repo_name);
        if temp.starts_with("~") {
            path = expand_sanitized_home(&temp);
        } else {
            path = temp;
        }
    }
    println!("Cloning into: {}", path.display());

    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
        newline: false,
    });
    //  Callback for outputing the progress of the clone operation.
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print(&mut state);
        true
    });

    //  Callback for outputing the progress of the checkout operation.
    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        print(&mut state);
    });

    //  Create the fetch options and the callback.
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    //  Build a repository providing the fetch options and the checkout options.
    //  Call clone to clone the repository.
    let res = RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(url, path.as_path());

    println!();

    match res {
        Ok(_repo) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

//
//
//  End of Sourced code https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs
//
//

/// Add files to the git repository.
///
/// repo: Repository instance.
/// paths: Path specs.
/// update: Update the index rather than
pub fn add_files(paths: &Vec<String>, update: Option<bool>) -> crate::Result<usize> {
    let repo = Repository::open(".").unwrap();
    let mut index = repo.index().unwrap();
    let is_update = update.unwrap_or(false);
    let items_added = RefCell::new(0_usize);
    let cb = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
        let status = repo.status_file(path).unwrap();
        let ret = if status.contains(git2::Status::WT_MODIFIED)
            || status.contains(git2::Status::WT_NEW)
            || status.contains(git2::Status::WT_RENAMED)
            || status.contains(git2::Status::WT_TYPECHANGE)
            || status.contains(git2::Status::WT_DELETED)
        {
            let mut cnt = items_added.borrow_mut();
            *cnt += 1;
            println!("Adding file: {}", path.display());
            println!("Status: {:?}", status);
            println!("File: {cnt:?}");
            0
        } else {
            1
        };
        ret
    };

    let cb = if !is_update {
        None
    } else {
        Some(cb as &mut git2::IndexMatchedPath)
    };

    //  Result of editing the index.

    let res: Result<(), Error> = if is_update {
        index.update_all(paths, cb)
    } else {
        index.add_all(paths, git2::IndexAddOption::DEFAULT, cb)
    };

    if res.is_err() {
        return Err(res.err().unwrap().into());
    }
    index.write().unwrap();

    Ok(*items_added.borrow())
}

/// Create a new commit that references the current HEAD.
pub fn create_commit(repo: &Repository, commit_msg: String) -> Result<Oid, git2::Error> {
    // Get the index and write it as a tree
    let mut index = repo.index()?;
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    // Get the HEAD reference and its commit
    let head = repo.head()?;
    let parent_commit = head.peel_to_commit()?;

    let name = crate::ENV.lock().unwrap().git_name.clone();
    let email = crate::ENV.lock().unwrap().git_email.clone();
    // Create a signature
    let sig = Signature::now(&name, &email)?;

    // Create the commit
    let commit_oid = repo.commit(
        Some("HEAD"),      // the name of the reference to update
        &sig,              // the author signature
        &sig,              // the committer signature
        &commit_msg,       // the commit message
        &tree,             // the tree object this commit points to
        &[&parent_commit], // parents of the commit
    )?;

    Ok(commit_oid)
}
