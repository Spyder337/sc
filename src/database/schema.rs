// @generated automatically by Diesel CLI.

diesel::table! {
    daily_quotes (id) {
        id -> Integer,
        quote_id -> Integer,
        time_stamp -> Timestamp,
    }
}

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
        time_stamp -> Timestamp,
    }
}

diesel::table! {
    task_relations (id) {
        id -> Integer,
        parent_id -> Integer,
        child_id -> Integer,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        task -> Text,
        desc -> Nullable<Text>,
        status -> Text,
        time_stamp -> Timestamp,
        due_date -> Nullable<Timestamp>,
        renewal_duration -> Nullable<Integer>,
    }
}

diesel::joinable!(daily_quotes -> quotes (quote_id));

diesel::allow_tables_to_appear_in_same_query!(
    daily_quotes,
    quotes,
    searches,
    task_relations,
    tasks,
);
