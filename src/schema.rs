// @generated automatically by Diesel CLI.

diesel::table! {
    repositories (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        is_private -> Bool,
        stars -> BigInt,
        forks -> BigInt,
        main_language -> Text,
        last_update -> Timestamp,
        owner_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repository_collaborators (id) {
        id -> Integer,
        repository_id -> Integer,
        user_id -> Integer,
        role -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repository_topics (id) {
        id -> Integer,
        repository_id -> Integer,
        topic -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        display_name -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        bio -> Nullable<Text>,
        location -> Nullable<Text>,
        website -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(repositories -> users (owner_id));
diesel::joinable!(repository_collaborators -> repositories (repository_id));
diesel::joinable!(repository_collaborators -> users (user_id));
diesel::joinable!(repository_topics -> repositories (repository_id));

diesel::allow_tables_to_appear_in_same_query!(
    repositories,
    repository_collaborators,
    repository_topics,
    users,
);
