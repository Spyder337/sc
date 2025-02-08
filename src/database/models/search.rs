use crate::database::schema::searches;
use chrono::NaiveDateTime;
use diesel::prelude::*;

/// A google search history item.
#[derive(Debug, Queryable, AsChangeset, Selectable, Clone, Insertable)]
#[diesel(table_name = searches)]
pub struct SearchEntry {
    pub id: i32,
    pub query: String,
    pub website: Option<String>,
    pub allintext: Option<String>,
    #[diesel(sql_type = Timestamp)]
    pub time_stamp: NaiveDateTime,
}

impl std::fmt::Display for SearchEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_fmt(format_args!("Query: {}\n", self.query));
        if let Some(website) = &self.website {
            let _ = f.write_fmt(format_args!("Website: {}\n", website));
        }
        if let Some(all_text) = &self.allintext {
            let _ = f.write_fmt(format_args!("All in text: {}\n", all_text));
        }
        let _ = f.write_fmt(format_args!("Time: {}\n", self.time_stamp));
        f.write_fmt(format_args!(
            "Query String: {}\n",
            crate::commands::web::core::query_string_builder(
                &self.query,
                &self.website,
                &self.allintext,
            )
        ))
    }
}
