// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Uuid,
        user_id -> Varchar,
        stream_id -> Uuid,
        content -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    favorites (user_id, stream_id) {
        user_id -> Varchar,
        stream_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    streams (id) {
        id -> Uuid,
        streamed_by -> Varchar,
        title -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_streaming -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        description -> Varchar,
        //created_at -> Timestamp,
        //updated_at -> Timestamp,
    }
}

diesel::joinable!(comments -> streams (stream_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(favorites -> streams (stream_id));
diesel::joinable!(favorites -> users (user_id));
diesel::joinable!(streams -> users (streamed_by));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    favorites,
    streams,
    users,
);
