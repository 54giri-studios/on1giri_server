use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{Channel, ChannelPermissions, ChannelPermissionsForRole, DbPool, ErrorResponse, Guild, JsonResponse, NewChannelPermissions, PopulatedChannelPermissions, Role};
use crate::schema::{
    channels::dsl as c,
    channel_permissions::dsl as cp,
    guilds::dsl as g,
    roles::dsl as r,
};


#[get("/<guild_id>/channels/<channel_id>/permissions")]
pub async fn get_guild_channel_permissions(
    pool: &State<DbPool>,
    guild_id: i32,
    channel_id: i32
) -> JsonResponse<Vec<ChannelPermissionsForRole>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into())
    };

    let parts: Result<Vec<(ChannelPermissions, Role)>, _> = cp::channel_permissions
        .filter(cp::guild_id.eq(guild_id))
        .filter(cp::channel_id.eq(channel_id))
        .inner_join(r::roles.on(r::id.eq(cp::role_id)))
        .get_results(&mut conn)
        .await;

    match parts {
        Ok(parts) => {
            let mut parts: Vec<ChannelPermissionsForRole> = parts
                .into_iter()
                .map(|(cp, r)| ChannelPermissionsForRole::new(cp, r))
                .collect();

            Ok(parts.into())
        },
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}

#[get("/<guild_id>/channels/<channel_id>/roles/<role_id>/permissions")]
pub async fn get_guild_channel_role_permissions(
    pool: &State<DbPool>,
    guild_id: i32,
    channel_id: i32,
    role_id: i32,
) -> JsonResponse<ChannelPermissions> {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    let maybe_perms: Result<ChannelPermissions, _> = cp::channel_permissions
        .filter(cp::role_id.eq(role_id))
        .filter(cp::guild_id.eq(guild_id))
        .filter(cp::channel_id.eq(channel_id))
        .get_result(&mut conn)
        .await;

    match maybe_perms {
        Ok(perms) => Ok(perms.into()),
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}

#[patch("/<guild_id>/channels/<channel_id>/roles/<role_id>/permissions", format = "json", data = "<new_channel_permissions>")]
pub async fn patch_guild_channel_role_permissions(
    pool: &State<DbPool>,
    guild_id: i32,
    channel_id: i32,
    role_id: i32,
    new_channel_permissions: Json<NewChannelPermissions>,
) -> JsonResponse<ChannelPermissions> {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    let new_channel_permissions = new_channel_permissions.into_inner();

    let maybe_chan_perms: Result<ChannelPermissions, _> = diesel::update(cp::channel_permissions)
        .filter(cp::role_id.eq(role_id))
        .filter(cp::guild_id.eq(guild_id))
        .filter(cp::channel_id.eq(channel_id))
        .set(&new_channel_permissions)
        .returning(ChannelPermissions::as_returning())
        .get_result(&mut conn)
        .await;

    match maybe_chan_perms {
        Ok(perms) => Ok(perms.into()),
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}