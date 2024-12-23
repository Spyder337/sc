#![allow(dead_code)]
pub mod model;
mod schema;
pub mod sqlite;

pub type DbResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
