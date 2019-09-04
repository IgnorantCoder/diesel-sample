table! {
    posts (id) {
        id -> Uuid,
        content -> Text,
        id_users -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        screen_name -> Text,
        created_at -> Timestamp,
    }
}

joinable!(posts -> users (id_users));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
