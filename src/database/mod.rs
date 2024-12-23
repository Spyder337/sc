#![allow(dead_code)]
pub mod model;
mod schema;
pub mod sqlite;

/// A wrapper for [std::result::Result] with a [Box]ed [std::error::Error].
pub type DbResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
