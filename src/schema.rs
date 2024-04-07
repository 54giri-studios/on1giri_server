// @generated automatically by Diesel CLI.

diesel::table! {
    channel_kinds (kind) {
        kind -> Varchar,
    }
}

diesel::table! {
    channels (id) {
        id -> Int4,
        guild_id -> Int4,
        name -> Varchar,
        kind -> Varchar,
    }
}

diesel::table! {
    guilds (id) {
        id -> Int4,
        name -> Varchar,
        owner_id -> Int4,
    }
}

diesel::table! {
    messages (id, channel_id) {
        id -> Int4,
        channel_id -> Int4,
        author_id -> Int4,
        content -> Varchar,
        creation_date -> Timestamptz,
    }
}

diesel::table! {
    roles (id, guild_id) {
        id -> Int4,
        guild_id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        password -> Varchar,
        user_type -> Int4,
        email -> Varchar,
    }
}

diesel::table! {
    users_metadata (id) {
        id -> Int4,
        username -> Varchar,
        discriminator -> Int2,
        last_check_in -> Timestamptz,
        picture -> Text,
        account_creation -> Timestamptz,
        description -> Text,
    }
}

diesel::joinable!(channels -> channel_kinds (kind));
diesel::joinable!(channels -> guilds (guild_id));
diesel::joinable!(guilds -> users (owner_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (author_id));
diesel::joinable!(roles -> guilds (guild_id));
diesel::joinable!(users_metadata -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    channel_kinds,
    channels,
    guilds,
    messages,
    roles,
    users,
    users_metadata,
);
