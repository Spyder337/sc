// @generated automatically by Diesel CLI.

diesel::table! {
    quotes (id) {
        id -> Integer,
        quote -> Text,
        author -> Text,
    }
}

diesel::table! {
    searches (id) {
        id -> Integer,
        query -> Text,
        website -> Nullable<Text>,
        allintext -> Nullable<Text>,
        time_stamp -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(quotes, searches,);
