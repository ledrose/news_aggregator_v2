// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        passwd_hash -> Varchar,
        role -> Int4,
    }
}
