use std::path::Path;

use lazy_static::lazy_static;

pub mod core;

lazy_static! {
    pub static ref GIT_DIR: String = String::from("/home/Code/");
    pub static ref GIT_AUTHOR: String = String::from("Spyder337");
    pub static ref GIT_EMAIL: String = String::from("owsley.wood@gmail.com");
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
