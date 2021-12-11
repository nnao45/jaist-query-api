table! {
    foot_stamp (id) {
        id -> Unsigned<Bigint>,
        user_id -> Bigint,
        latitude -> Double,
        longitude -> Double,
        created_at -> Datetime,
    }
}

table! {
    post (id) {
        id -> Unsigned<Bigint>,
        title -> Varchar,
        body -> Text,
        user_id -> Bigint,
        published -> Bool,
        created_at -> Datetime,
    }
}

table! {
    user (id) {
        id -> Bigint,
        name -> Varchar,
        description -> Text,
        created_at -> Datetime,
    }
}

joinable!(foot_stamp -> user (user_id));
joinable!(post -> user (user_id));

allow_tables_to_appear_in_same_query!(
    foot_stamp,
    post,
    user,
);
