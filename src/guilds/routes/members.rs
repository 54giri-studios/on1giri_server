use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{DbPool, ErrorResponse, Guild, Member, PopulatedMember, Role, User, UserMetadata};

use crate::schema::{
    users_metadata::dsl as um,
    members::dsl as m,
    members_roles::dsl as mr,
    guilds::dsl as g,
    roles::dsl as r,
};

#[get("/<guild_id>/members/<member_id>")]
pub async fn get_member(
    pool: &State<DbPool>,
    guild_id: i32, 
    member_id: i32
) -> Result<Json<PopulatedMember>, Json<ErrorResponse>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    let parts: Result<(Guild, Member, UserMetadata), _> = g::guilds
        .filter(g::id.eq(guild_id))
        .inner_join(m::members)
        .filter(m::user_id.eq(member_id))
        .inner_join(um::users_metadata.on(um::id.eq(m::user_id)))
        .get_result(&mut conn)
        .await;

    let (guild, member, user_metadata) = match parts {
        Ok((g, m, um)) => (g, m, um),
        Err(err) => return Err(ErrorResponse::from(err).into())
    };

    let maybe_roles: Result<Vec<Role>, _> = r::roles
        .filter(r::guild_id.eq(guild_id))
        .inner_join(mr::members_roles)
        .filter(mr::member_id.eq(member_id))
        .select(Role::as_select())
        .get_results(&mut conn)
        .await;

    let roles = match maybe_roles {
        Ok(roles) => roles,
        Err(err) => return Err(ErrorResponse::from(err).into()),
    };

    Ok(PopulatedMember::new(user_metadata, guild, roles).into())
}