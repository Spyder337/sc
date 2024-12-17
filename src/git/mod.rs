#![allow(unused_imports, dead_code)]
use std::path::Path;

use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod core;

lazy_static! {
    pub static ref GIT_DIR: Mutex<String> = Mutex::new(String::from("/home/Code/"));
    pub static ref GIT_AUTHOR: Mutex<String> = Mutex::new(String::from("Spyder337"));
    pub static ref GIT_EMAIL: Mutex<String> = Mutex::new(String::from("owsley.wood@gmail.com"));
}

pub fn init() {
    let db = crate::Database::new("rsrc/database.db").unwrap();
    let _ = db.create_table();
    let g_path = db
        .get_item_by_var("GIT_DIR")
        .unwrap()
        .val
        .unwrap_or(String::from("~/Code"));
    let author = db
        .get_item_by_var("GIT_AUTHOR")
        .unwrap()
        .val
        .unwrap_or(String::from("Author"));
    let email = db
        .get_item_by_var("GIT_EMAIL")
        .unwrap()
        .val
        .unwrap_or(String::from("user.name@email.com"));
    println!(
        "Data\nPath: {}\nAuthor: {}\nEmail: {}",
        g_path, author, email
    );
}

pub fn get_git_dir() -> String {
    return GIT_DIR.lock().unwrap().clone();
}

pub fn set_git_dir(path: &str) -> bool {
    let db = crate::Database::new("rsrc/database.db").unwrap();
    let res = db.insert_or_update_item("GIT_DIR", Some(path));
    if res.is_ok() {
        *GIT_DIR.lock().unwrap() = path.to_string();
        let _ = db.update_database_file();
        return true;
    }
    false
}

pub fn get_git_author() -> String {
    return GIT_AUTHOR.lock().unwrap().clone();
}

pub fn set_git_author(author: &str) -> bool {
    let db = crate::Database::new("rsrc/database.db").unwrap();
    let res = db.insert_or_update_item("GIT_AUTHOR", Some(author));
    if res.is_ok() {
        *GIT_AUTHOR.lock().unwrap() = author.to_string();
        let _ = db.update_database_file();
        return true;
    }
    false
}

pub fn get_git_email() -> String {
    return GIT_EMAIL.lock().unwrap().clone();
}

pub fn set_git_email(email: &str) -> bool {
    let db = crate::Database::new("rsrc/database.db").unwrap();
    let res = db.insert_or_update_item("GIT_EMAIL", Some(email));

    if let Ok(updated) = res {
        *GIT_EMAIL.lock().unwrap() = email.to_string();
        let _ = db.update_database_file();
        println!("Email set to: {}", GIT_EMAIL.lock().unwrap());
        true
    } else {
        false
    }
}
