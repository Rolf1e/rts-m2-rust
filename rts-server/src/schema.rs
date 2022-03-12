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
    matchs (id) {
        id -> Int4,
        winner -> Int4,
        looser -> Int4,
        score_winner -> Int4,
        score_looser -> Int4,
    }
}

joinable!(ais -> users (owner));
joinable!(matchs -> users (winner));

allow_tables_to_appear_in_same_query!(users, ais, matchs, );
