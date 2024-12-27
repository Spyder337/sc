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
        time_stamp -> Date,
        due_date -> Nullable<Date>,
        renewal_duration -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    quotes,
    searches,
    task_relations,
    tasks,
);
