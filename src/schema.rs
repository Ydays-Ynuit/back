// @generated automatically by Diesel CLI.
diesel::table! {
    friends (id) {
        id -> Unsigned<Bigint>,
        user_id_sender -> Unsigned<Bigint>,
        user_id_receiver -> Unsigned<Bigint>,
        status -> Tinyint,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        public_key -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(friends, users,);
