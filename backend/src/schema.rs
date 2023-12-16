// @generated automatically by Diesel CLI.

diesel::table! {
    news (id) {
        id -> Int4,
        header -> Varchar,
        source_id -> Int4,
        theme_id -> Int4,
        text -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    sources (id) {
        id -> Int4,
        name -> Varchar,
        source_type -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
    }
}

diesel::table! {
    sourcethemes (id) {
        id -> Int4,
        source_id -> Int4,
        theme_id -> Int4,
        source_theme_name -> Varchar,
    }
}

diesel::table! {
    themes (id) {
        id -> Int4,
        theme_name -> Varchar,
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

diesel::joinable!(news -> sources (source_id));
diesel::joinable!(news -> sourcethemes (theme_id));
diesel::joinable!(sourcethemes -> sources (source_id));
diesel::joinable!(sourcethemes -> themes (theme_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    news,
    roles,
    sources,
    sourcethemes,
    themes,
    users,
);
