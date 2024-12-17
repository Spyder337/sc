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
    let mut db = crate::Database::new("rsrc/database.db").unwrap();
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

pub fn set_git_dir(path: &String) -> bool {
    let mut db = crate::Database::new("rsrc/database.db").unwrap();
    let res = db.insert_item("GIT_DIR", Some(path));
    if res.is_ok() {
        *GIT_DIR.lock().unwrap() = path.clone();
        return true;
    }
    false
}

pub fn get_git_author() -> String {
    return GIT_AUTHOR.lock().unwrap().clone();
}

pub fn set_git_author(author: &String) -> bool {
    let mut db = crate::Database::new("rsrc/database.db").unwrap();
    let res = db.insert_item("GIT_AUTHOR", Some(author));
    if res.is_ok() {
        *GIT_AUTHOR.lock().unwrap() = author.clone();
        return true;
    }
    false
}

pub fn get_git_email() -> String {
    return GIT_EMAIL.lock().unwrap().clone();
}

pub fn set_git_email(email: &String) -> bool {
    let mut db = crate::Database::new("rsrc/database.db").unwrap();
    let res = db.insert_item("GIT_EMAIL", Some(email));
    if res.is_ok() {
        *GIT_EMAIL.lock().unwrap() = email.clone();
        return true;
    }
    false
}
