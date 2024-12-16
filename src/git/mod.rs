use std::path::Path;

use lazy_static::lazy_static;

pub mod core;

lazy_static! {
    pub static ref GIT_DIR: &'static Path = &Path::new("/home/Code/");
    pub static ref GIT_AUTHOR: String = String::from("Spyder337");
    pub static ref GIT_EMAIL: String = String::from("owsley.wood@gmail.com");
}
