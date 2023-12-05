// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        date -> Timestamp,
    }
}
