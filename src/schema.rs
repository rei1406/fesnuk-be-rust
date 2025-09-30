// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ltree"))]
    pub struct Ltree;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Ltree;

    comments (id) {
        id -> Int4,
        post_id -> Int4,
        path -> Ltree,
        content -> Text,
        attachments -> Jsonb,
        reactions -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    nooks (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        attachments -> Jsonb,
        reactions -> Jsonb,
        nook_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(posts -> nooks (nook_id));

diesel::allow_tables_to_appear_in_same_query!(comments, nooks, posts,);
