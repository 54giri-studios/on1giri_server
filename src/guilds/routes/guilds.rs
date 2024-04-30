use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{Channel, DbPool, ErrorResponse, Guild, JsonResponse, PatchGuild, PopulatedGuild, Role, UserMetadata};

use crate::schema::{
    channels::dsl as c,
    guilds::dsl as g,
    roles::dsl as r,
    users_metadata::dsl as um,
};



#[get("/<guild_id>")]
pub async fn get_guild(
    pool: &State<DbPool>,
    guild_id: i32
) -> JsonResponse<PopulatedGuild> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    let parts: Result<(Guild, UserMetadata), _> = g::guilds
        .filter(g::id.eq(guild_id))
        .inner_join(um::users_metadata.on(g::owner_id.eq(um::id)))
        .get_result(&mut conn)
        .await;

    let (guild, owner) = match parts {
        Ok((g, um)) => (g, um),
        Err(err) => return Err(ErrorResponse::from(err).into()),
    };

    let roles: Vec<Role> = super::get_guild_roles(pool, guild_id).await?.into_inner();
    let channels: Vec<Channel> = super::get_guild_channels(pool, guild_id).await?.into_inner();

    let populated_guild = PopulatedGuild::new(guild, owner, roles, channels);

    Ok(populated_guild.into())
}

#[patch("/<guild_id>", data = "<guild_patch>", format = "json")]
pub async fn patch_guild<'a>(
    pool: &State<DbPool>,
    guild_id: i32,
    guild_patch: Json<PatchGuild<'a>>
) -> JsonResponse<PopulatedGuild> {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(err) => return Err(ErrorResponse::internal_error(err).into())
    };

    let a = diesel::update(g::guilds)
        .filter(g::id.eq(guild_id))
        .set(guild_patch.into_inner())
        .execute(&mut conn)
        .await;

    get_guild(pool, guild_id).await
}
