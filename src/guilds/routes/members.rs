use diesel::prelude::*;
use diesel::result::{
    Error as DieselError,
    DatabaseErrorKind,
};
use diesel_async::{scoped_futures::ScopedFutureExt, RunQueryDsl};

use rocket::http::Status;
use rocket::{serde::json::Json, State};

use crate::{MemberRole, RoleCategory};
use crate::{DbPool, ErrorResponse, Guild, JsonResponse, Member, PopulatedGuild, PopulatedMember, Role, User, UserMetadata};

use crate::schema::{
    users_metadata::dsl as um,
    members::dsl as m,
    members_roles::dsl as mr,
    guilds::dsl as g,
    roles::dsl as r,
};

#[get("/<guild_id>/members/<member_id>")]
pub async fn get_guild_member(
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

#[get("/<guild_id>/members")]
pub async fn get_guild_members(
    pool: &State<DbPool>,
    guild_id: i32,
) -> JsonResponse<Vec<PopulatedMember>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    let maybe_guild: Result<Guild, _> = g::guilds
        .filter(g::id.eq(guild_id))
        .get_result(&mut conn)
        .await;

    let guild = match maybe_guild {
        Ok(g) => g,
        Err(err) => return Err(ErrorResponse::from(err).into())
    };

    let maybe_members: Result<Vec<(Member, UserMetadata)>, _> = m::members
        .filter(m::guild_id.eq(guild_id))
        .inner_join(um::users_metadata.on(um::id.eq(m::user_id)))
        .select((Member::as_select(), UserMetadata::as_select()))
        .get_results(&mut conn)
        .await;

    let rich_members = match maybe_members {
        Ok(rm) => rm,
        Err(err) => return Err(ErrorResponse::from(err).into()),
    };

    let mut populated_members: Vec<PopulatedMember> = Vec::with_capacity(rich_members.len());

    for (member, user_meta) in rich_members.into_iter() {

        let maybe_roles: Result<Vec<Role>, _> = r::roles
            .filter(r::guild_id.eq(guild_id))
            .inner_join(mr::members_roles.on(
                mr::role_id.eq(r::id)
                .and(mr::guild_id.eq(r::guild_id))
            ))
            .filter(mr::member_id.eq(member.user_id()))
            .select(Role::as_select())
            .get_results(&mut conn)
            .await;

        let roles = match maybe_roles {
            Ok(roles) => roles,
            Err(err) => return Err(ErrorResponse::from(err).into())
        }; 
        let populated_member = PopulatedMember::new(user_meta, guild.clone(), roles);

        // That's exactly the sort of stuff that should 
        // have been done with an event stream ...
        populated_members.push(populated_member);
    }

    Ok(populated_members.into())
}

#[post("/<guild_id>/members/<member_id>/create")]
pub async fn post_guild_member(
    pool: &State<DbPool>,
    guild_id: i32,
    member_id: i32
) -> JsonResponse<PopulatedMember> {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(err) => return Err(ErrorResponse::internal_error(err).into())
    };

    let maybe_pop_member: Result<PopulatedMember, ErrorResponse> = conn
        .build_transaction()
        .serializable()
        .run(move |conn| async move {
            let member: Member = Member::new(member_id, guild_id);
            diesel::insert_into(m::members)
                .values(&member)
                .execute(conn)
                .await
                .map_err(|err| match err {
                    DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, info) => {
                        ErrorResponse::new(
                            Status::BadRequest, 
                            format!("One of the provided ids is invalid: {info:?}")
                        )
                    },
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                        ErrorResponse::new(
                            Status::Conflict,
                            format!("User {} is already a member of guild {}", member_id, guild_id)
                        )
                    },
                    other => ErrorResponse::internal_error(other)
                })?;

            let guild: Guild = g::guilds
                .filter(g::id.eq(member.guild_id))
                .get_result(conn)
                .await
                .map_err(ErrorResponse::from)?;

            let default_role: Role = r::roles
                .filter(r::guild_id.eq(guild.id()))
                .filter(r::category.eq(RoleCategory::everyone().to_string()))
                .get_result(conn)
                .await
                .map_err(ErrorResponse::from)?;

            diesel::insert_into(mr::members_roles)
                .values(MemberRole::new(default_role.id(), guild.id(), member.user_id()))
                .execute(conn)
                .await
                .map_err(ErrorResponse::from)?;

            let user_meta: UserMetadata = um::users_metadata
                .filter(um::id.eq(member.user_id()))
                .get_result(conn)
                .await
                .map_err(ErrorResponse::from)?;

            let pop_member: PopulatedMember = PopulatedMember::new(
                user_meta,
                guild,
                vec![default_role],
            );

            Ok(pop_member)
        }.scope_boxed())
        .await;

    match maybe_pop_member {
        Ok(pm) => Ok(pm.into()),
        Err(err) => Err(err.into())
    }
}
