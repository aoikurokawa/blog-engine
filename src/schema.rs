table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        category_id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        create_at -> Timestamp,
    }
}

joinable!(posts -> categories (category_id));

allow_tables_to_appear_in_same_query!(
    categories,
    posts,
    users,
);
