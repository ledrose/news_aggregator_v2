// @generated automatically by Diesel CLI.

diesel::table! {
    feeds (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    feedsource (id) {
        id -> Int4,
        feed_id -> Int4,
        source_theme_id -> Int4,
    }
}

diesel::table! {
    news (id) {
        id -> Int4,
        header -> Varchar,
        date_time -> Timestamptz,
        source_id -> Int4,
        theme_id -> Int4,
        description -> Nullable<Varchar>,
        link -> Varchar,
        image -> Nullable<Varchar>,
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

diesel::joinable!(feeds -> users (user_id));
diesel::joinable!(feedsource -> feeds (feed_id));
diesel::joinable!(feedsource -> sourcethemes (source_theme_id));
diesel::joinable!(news -> sources (source_id));
diesel::joinable!(news -> sourcethemes (theme_id));
diesel::joinable!(sourcethemes -> sources (source_id));
diesel::joinable!(sourcethemes -> themes (theme_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    feeds,
    feedsource,
    news,
    roles,
    sources,
    sourcethemes,
    themes,
    users,
);
