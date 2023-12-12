// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        passwd_hash -> Varchar,
        role_id -> Int4,
    }
}

diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
