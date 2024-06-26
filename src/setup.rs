use chrono::DateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{http::CookieJar, local::asynchronous::Client};

use crate::{AccessLevel, Channel, ChannelKind, ChannelPermissions, Color, DbPool, Guild, Member, MemberRole, Role, RoleCategory, TokenHandler, User, UserMetadata};

use std::env::var;

use crate::schema::{
    channels::dsl as c_dsl,
    channel_permissions::dsl as cp_dsl,
    guilds::dsl as g_dsl,
    members::dsl as m_dsl,
    members_roles::dsl as mr_dsl,
    roles::dsl as r_dsl,
    users::dsl as u_dsl,
    users_metadata::dsl as um_dsl,
};

/*
pub async fn setup_system(pool: &DbPool, token_handler: &TokenHandler) -> Result<(), Box<dyn std::error::Error>> {

    // User
    let overlord = User::new(
        0,
        var("OVERLORD_PASSWORD").expect("OVERLORD_PASSWORD must be set"),
        AccessLevel::admin().to_string(),
        var("OVERLORD_EMAIL").expect("OVERLORD_EMAIL must be set"),
    );

    diesel::insert_into(u_dsl::users)
        .values(&overlord)
        .on_conflict(u_dsl::id)
        .do_update()
        .set(&overlord)
        .execute(&mut conn)
        .await?;

    // UserMetadata
    let overlord_metadata = UserMetadata::new(
        0,
        var("OVERLORD_USERNAME").expect("OVERLORD_USERNAME must be set"),
        var("OVERLORD_DISCRIMINATOR")
            .expect("OVERLORD_DICRIMINATOR must be set")
            .parse()
            .expect("OVERLORD_DISCRIMINATOR must be a valid i16 between 0000 and 9999"),
        DateTime::from_timestamp(
            var("OVERLORD_LAST_CHECK_IN")
                .expect("OVERLORD_LAST_CHECK_IN must be set")
                .parse()
                .expect("OVERLORD_LAST_CHECK_IN must be a valid timestamp"), 
            0
        ).expect("OVERLORD_LAST_CHECK_IN must be a valid timestamp"),
        var("OVERLORD_PICTURE").expect("OVERLORD_PICTURE must be set"),
        DateTime::from_timestamp(
            var("OVERLORD_ACCOUNT_CREATION")
                .expect("OVERLORD_ACCOUNT_CREATION must be set")
                .parse()
                .expect("OVERLORD_ACCOUNT_CREATION must be a valid timestamp"), 
            0
        ).expect("OVERLORD_ACCOUNT_CREATION must be a valid timestamp"),
        var("OVERLORD_DESCRIPTION").expect("OVERLORD_DESCRIPTION must be set"),
    );

    diesel::insert_into(um_dsl::users_metadata)
        .values(&overlord_metadata)
        .on_conflict((um_dsl::username, um_dsl::discriminator))
        .do_update()
        .set(&overlord_metadata)
        .execute(&mut conn)
        .await?;

    // System guild
    let system_guild = Guild::new(
        0,
        var("TOMB_NAME").expect("TOMB_NAME must be set"),
        0,
        var("TOMB_DESCRIPTION").expect("TOMB_DESCRIPTION must be set"),
        DateTime::from_timestamp(
            var("TOMB_CREATION_DATE")
                .expect("TOMB_CREATION_DATE must be set")
                .parse()
                .expect("TOMB_CREATION_DATE must be a valid integer")
                ,
            0
        ).expect("TOMB_CREATION_DATE must ba a valid unix timestamp"),
    );

    diesel::insert_into(g_dsl::guilds)
        .values(&system_guild)
        .on_conflict(g_dsl::id)
        .do_update()
        .set(&system_guild)
        .execute(&mut conn)
        .await?;

    // Make the overlord part of the guild
    let overlord_member = Member::new(0, 0);
    diesel::insert_into(m_dsl::members)
        .values(&overlord_member)
        .on_conflict((m_dsl::user_id, m_dsl::guild_id))
        .do_update()
        .set(&overlord_member)
        .execute(&mut conn)
        .await;

    // System channel
    let system_channel = Channel::new(
        0,
        0,
        var("NEXUS_NAME").expect("NEXUS_NAME must be set"),
        ChannelKind::text().to_string()
    );

    diesel::insert_into(c_dsl::channels)
        .values(&system_channel)
        .on_conflict(c_dsl::id)
        .do_update()
        .set(&system_channel)
        .execute(&mut conn)
        .await?;

    // Supreme overlord role
    let supreme_overlord = Role::new(
        0,
        0,
        var("OVERLORD_ROLE_NAME").expect("OVERLORD_ROLE_NAME must be set "),

        Color::from_hex(var("OVERLORD_ROLE_COLOR").expect("OVERLORD_ROLE_COLOR must be set"))
            .expect("OVERLORD_ROLE_COLOR must be a valid hex color")
            .to_hex_string(),
        RoleCategory::owner().to_string(),
    );

    diesel::insert_into(r_dsl::roles)
        .values(&supreme_overlord)
        .on_conflict(r_dsl::id)
        .do_update()
        .set(&supreme_overlord)
        .execute(&mut conn)
        .await?;

    // Attribute that role to the overlord
    let overlord_role = MemberRole::new(0, 0, 0);

    diesel::insert_into(mr_dsl::members_roles)
        .values(&overlord_role)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;

    // Give him every right
    let overlord_role_permissions = ChannelPermissions::all_allowed(0, 0, 0);
    diesel::insert_into(cp_dsl::channel_permissions)
        .values(&overlord_role_permissions)
        .on_conflict((cp_dsl::role_id, cp_dsl::guild_id, cp_dsl::channel_id))
        .do_update()
        .set(&overlord_role_permissions)
        .execute(&mut conn)
        .await?;

    let overseer = User::new(
        1,
        var("OVERSEER_PASSWORD").expect("OVERSEER_PASSWORD must be set"),
        AccessLevel::admin().to_string(),
        var("OVERSEER_EMAIL").expect("OVERSEER_EMAIL must be set"),
    );

    diesel::insert_into(u_dsl::users)
        .values(&overseer)
        .on_conflict(u_dsl::id)
        .do_update()
        .set(&overseer)
        .execute(&mut conn)
        .await?;

    // UserMetadata
    let overseer_metadata = UserMetadata::new(
        1,
        var("OVERSEER_USERNAME").expect("OVERSEER_USERNAME must be set"),
        var("OVERSEER_DISCRIMINATOR")
            .expect("OVERSEER_DICRIMINATOR must be set")
            .parse()
            .expect("OVERSEER_DISCRIMINATOR must be a valid i16 between 0000 and 9999"),
        DateTime::from_timestamp(
            var("OVERSEER_LAST_CHECK_IN")
                .expect("OVERSEER_LAST_CHECK_IN must be set")
                .parse()
                .expect("OVERSEER_LAST_CHECK_IN must be a valid timestamp"), 
            0
        ).expect("OVERSEER_LAST_CHECK_IN must be a valid timestamp"),
        var("OVERSEER_PICTURE").expect("OVERSEER_PICTURE must be set"),
        DateTime::from_timestamp(
            var("OVERSEER_ACCOUNT_CREATION")
                .expect("OVERSEER_ACCOUNT_CREATION must be set")
                .parse()
                .expect("OVERSEER_ACCOUNT_CREATION must be a valid timestamp"), 
            0
        ).expect("OVERSEER_ACCOUNT_CREATION must be a valid timestamp"),
        var("OVERSEER_DESCRIPTION").expect("OVERSEER_DESCRIPTION must be set"),
    );

    diesel::insert_into(um_dsl::users_metadata)
        .values(&overseer_metadata)
        .on_conflict((um_dsl::username, um_dsl::discriminator))
        .do_update()
        .set(&overseer_metadata)
        .execute(&mut conn)
        .await?;

    // Make the overseer part of the guild
    let overseer_member = Member::new(1, 0);
    diesel::insert_into(m_dsl::members)
        .values(&overseer_member)
        .on_conflict((m_dsl::user_id, m_dsl::guild_id))
        .do_update()
        .set(&overseer_member)
        .execute(&mut conn)
        .await;

    diesel::insert_into(c_dsl::channels)
        .values(&system_channel)
        .on_conflict(c_dsl::id)
        .do_update()
        .set(&system_channel)
        .execute(&mut conn)
        .await?;

    // Supreme overseer role
    let supreme_overseer = Role::new(
        1,
        0,
        var("OVERSEER_ROLE_NAME").expect("overseer_ROLE_NAME must be set "),

        Color::from_hex(var("OVERSEER_ROLE_COLOR").expect("overseer_ROLE_COLOR must be set"))
            .expect("OVERSEER_ROLE_COLOR must be a valid hex color")
            .to_hex_string(),
        RoleCategory::standard().to_string(),
    );

    diesel::insert_into(r_dsl::roles)
        .values(&supreme_overseer)
        .on_conflict(r_dsl::id)
        .do_update()
        .set(&supreme_overseer)
        .execute(&mut conn)
        .await?;

    // Attribute that role to the overseer
    let overseer_role = MemberRole::new(1, 0, 1);

    diesel::insert_into(mr_dsl::members_roles)
        .values(&overseer_role)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;

    // Give her no rights for testing purposes
    let overseer_role_permissions = ChannelPermissions::nothing_allowed(1, 0, 0);
    diesel::insert_into(cp_dsl::channel_permissions)
        .values(&overseer_role_permissions)
        .on_conflict((cp_dsl::role_id, cp_dsl::guild_id, cp_dsl::channel_id))
        .do_update()
        .set(&overseer_role_permissions)
        .execute(&mut conn)
        .await?;

    Ok(())
}
*/