table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        email -> Text,
    }
}

table! {
    ais (id) {
        id -> Int4,
        owner -> Int4,
        code -> Text,
    }
}

joinable!(ais -> users (owner));

allow_tables_to_appear_in_same_query!(users, ais,);
