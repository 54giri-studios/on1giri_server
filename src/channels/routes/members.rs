use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use itertools::Itertools;
use rocket::{serde::json::Json, State};

use crate::{Channel, ChannelPermissions, DbPool, ErrorResponse, Guild, Member, MemberRole, PopulatedMember, Role, UserMetadata};

#[get("/<channel_id>/members")]
pub async fn get_channel_members(
    pool: &State<DbPool>, 
    channel_id: i32
) -> Result<Json<Vec<PopulatedMember>>, Json<ErrorResponse>>{
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    use crate::schema::{
        users_metadata::dsl as um_dsl,
        members::dsl as m_dsl,
        members_roles::dsl as mr_dsl,
        channel_permissions::dsl as cp_dsl,
        roles::dsl as r_dsl,
        guilds::dsl as g_dsl,
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
