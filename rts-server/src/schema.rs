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

table! {
    leader_board (id) {
        id -> Int4,
        rank -> Int4,
        player -> Int4,
        score -> Int4,
        wins -> Int4,
        losses -> Int4,
    }
}

joinable!(ais -> users (owner));
joinable!(leader_board -> users (player));

allow_tables_to_appear_in_same_query!(users, ais, leader_board, );
