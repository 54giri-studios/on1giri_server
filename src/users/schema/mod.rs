diesel::table! {
    users (id) {
        id -> Int4,
        password -> VarChar,
        user_type -> Int4,
        email -> VarChar,
    }
}

diesel::table! {
    users_metadata (id) {
        id -> Int4,
        username -> VarChar,
        discriminator -> Int2,
        last_check_in -> Timestamptz,
        picture -> Text,
        account_creation -> Timestamptz,
        description -> Text,
    }
}
