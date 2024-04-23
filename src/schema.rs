// @generated automatically by Diesel CLI.

diesel::table! {
    access_levels (level) {
        level -> Varchar,
    }
}

diesel::table! {
    channel_kinds (kind) {
        kind -> Varchar,
    }
}

diesel::table! {
    channel_permissions (role_id, guild_id, channel_id) {
        role_id -> Int4,
        guild_id -> Int4,
        channel_id -> Int4,
        can_read -> Bool,
        can_write -> Bool,
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
        description -> Text,
        creation_date -> Timestamptz,
    }
}

diesel::table! {
    members (user_id, guild_id) {
        user_id -> Int4,
        guild_id -> Int4,
    }
}

diesel::table! {
    members_roles (role_id, guild_id, member_id) {
        role_id -> Int4,
        guild_id -> Int4,
        member_id -> Int4,
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
    roles (id) {
        id -> Int4,
        guild_id -> Int4,
        name -> Varchar,
        #[max_length = 7]
        color -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        password -> Varchar,
        access_level -> Varchar,
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

diesel::joinable!(channel_permissions -> channels (channel_id));
diesel::joinable!(channel_permissions -> guilds (guild_id));
diesel::joinable!(channel_permissions -> roles (role_id));
diesel::joinable!(channels -> channel_kinds (kind));
diesel::joinable!(channels -> guilds (guild_id));
diesel::joinable!(guilds -> users (owner_id));
diesel::joinable!(members -> guilds (guild_id));
diesel::joinable!(members -> users (user_id));
diesel::joinable!(members_roles -> roles (role_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (author_id));
diesel::joinable!(roles -> guilds (guild_id));
diesel::joinable!(users -> access_levels (access_level));
diesel::joinable!(users_metadata -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    access_levels,
    channel_kinds,
    channel_permissions,
    channels,
    guilds,
    members,
    members_roles,
    messages,
    roles,
    users,
    users_metadata,
);
