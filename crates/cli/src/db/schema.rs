// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    secrets (id) {
        id -> Text,
        name -> Text,
        project_id -> Text,
        config -> Text,
        secret -> Binary,
        nonce -> Binary,
        created_at -> Timestamp,
    }
}

diesel::joinable!(secrets -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(projects, secrets,);
