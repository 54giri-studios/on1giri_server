use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{scoped_futures::ScopedFutureExt, RunQueryDsl};

use rocket::http::Status;
use itertools::Itertools;
use rocket::{serde::json::Json, State};

use crate::{
    Channel, 
    ChannelPermissions, 
    DbPool, 
    ErrorResponse, 
    Guild, 
    JsonResponse, 
    Member, 
    MemberRole, 
    PopulatedMember, 
    Role, 
    UserMetadata
};

use crate::schema::{
    users_metadata::dsl as um,
    members::dsl as m,
    members_roles::dsl as mr,
    channels::dsl as c,
    channel_permissions::dsl as cp,
    roles::dsl as r,
    guilds::dsl as g,
};

#[get("/<channel_id>/members")]
pub async fn get_channel_members(
    pool: &State<DbPool>, 
    channel_id: i32
) -> JsonResponse<Vec<PopulatedMember>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    let maybe_members: Result<Vec<PopulatedMember>, ErrorResponse> = conn
        .build_transaction()
        .serializable()
        .run(move |conn| async move {
            let (channel, guild): (Channel, Guild) = c::channels
                .filter(c::id.eq(channel_id))
                .inner_join(g::guilds.on(g::id.eq(c::guild_id)))
                .get_result(conn)
                .await
                .map_err(
                    |err| match err {
                        DieselError::NotFound => {
                            ErrorResponse::new(
                                Status::NotFound, 
                                format!("Provided channel {channel_id} was not found")
                            )
                        },
                        other => ErrorResponse::from(other)
                    }
                )?; 

            let filtered_members: Vec<(Member, UserMetadata)> = m::members
                .filter(m::guild_id.eq(channel.guild_id()))
                .inner_join(um::users_metadata.on(um::id.eq(m::user_id)))
                .get_results(conn)
                .await
                .map_err(ErrorResponse::from)?;

            let mut populated_members: Vec<PopulatedMember> = Vec::with_capacity(filtered_members.len());

            for (member, user_meta) in filtered_members.into_iter() {
                let roles: Vec<(Role, bool)> = mr::members_roles
                    .filter(mr::guild_id.eq(channel.guild_id()))
                    .filter(mr::member_id.eq(member.user_id()))
                    .inner_join(
                        r::roles
                            .on(
                                r::id
                                    .eq(mr::role_id)
                                    .and(r::guild_id.eq(mr::guild_id))
                            )
                    )
                    .inner_join(cp::channel_permissions.on(cp::role_id.eq(r::id)))
                    .filter(cp::channel_id.eq(channel.id()))
                    .filter(cp::guild_id.eq(channel.guild_id()))
                    .select((Role::as_select(), cp::can_read))
                    .order_by(r::id.desc())
                    .get_results(conn)
                    .await
                    .map_err(ErrorResponse::from)?;

                let roles = roles
                    .into_iter()
                    .filter_map(|(role, can_read)| if can_read {Some(role) } else { None })
                    .collect();

                populated_members.push(PopulatedMember::new(user_meta, guild.clone(), roles));
            }
            Ok(populated_members)
        }.scope_boxed()
    ).await;

    match maybe_members {
        Ok(m) => Ok(m.into()),
        Err(e) => Err(e.into())
    }

}

/*
#[get("/<channel_id>/members")]
pub async fn get_channel_members(
    pool: &State<DbPool>, 
    channel_id: i32
) -> Result<Json<Vec<PopulatedMember>>, Json<ErrorResponse>>{
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };


    let mut parts: Result<Vec<(Guild, UserMetadata, Role)>, _> = cp_dsl::channel_permissions
        .filter(
            cp_dsl::channel_id.eq(channel_id)
                .and(cp_dsl::can_read).eq(true)
        )
        .inner_join(r_dsl::roles)
        .inner_join(g_dsl::guilds)
        .inner_join(
            mr_dsl::members_roles
                .on(
                    mr_dsl::role_id.eq(cp_dsl::role_id)
                        .and(
                            mr_dsl::guild_id.eq(cp_dsl::guild_id)
                        )
                )
        )
        .inner_join(
            m_dsl::members
                .on(m_dsl::user_id.eq(mr_dsl::member_id)
            )
        )
        .inner_join(
            um_dsl::users_metadata
                .on(um_dsl::id.eq(m_dsl::user_id))
        )
        .select((Guild::as_select(), UserMetadata::as_select(), Role::as_select()))
        .get_results(&mut conn)
        .await;

    match parts {
        Err(err) => Err(ErrorResponse::from(err).into()),
        Ok(parts) => {
            // Please don't ask me whatever is going on here
            // But basically, we got tuples (nth user, guild, pth role)
            // And we want to turn it into (guild, (nth user, Vec<roles>)) to build 
            // our populated member
            let members: Vec<PopulatedMember> = parts
                .into_iter()
                .group_by(|(g, um, r)| (um.clone(), g.clone()))
                .into_iter()
                .map(|(member_info, group)| {
                    let (user_meta, guild) = member_info;
                    let roles: Vec<Role> = group.into_iter().map(|(_g, _um, role)| role).collect();

                    PopulatedMember::new(user_meta.clone(), guild.clone(), roles)
                })
                .collect();

            Ok(members.into())
        }
    }
}
*/
