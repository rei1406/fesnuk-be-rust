// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        parent_id -> Nullable<Int4>,
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
        image -> Nullable<Varchar>,
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
