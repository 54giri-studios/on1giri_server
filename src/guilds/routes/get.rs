use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State}; 
use crate::{Channel, ChannelPermissions, DbPool, ErrorResponse, Guild, PopulatedChannelPermissions, PopulatedMessage, Role};

#[get("/<user_id>/guilds")]
pub async fn get_guilds(pool: &State<DbPool>, user_id: i32) -> Result<Json<Vec<Guild>>, Json<ErrorResponse>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(ErrorResponse::internal_error(err).into())
        }
    };

    use crate::schema::members::dsl as m_dsl;
    use crate::schema::guilds::dsl as g_dsl;

    let maybe_guilds: Result<Vec<Guild>, _> = m_dsl::members
        .inner_join(g_dsl::guilds)
        .select(Guild::as_select())
        .filter(m_dsl::user_id.eq(user_id))
        .get_results(&mut conn)
        .await;

    match maybe_guilds {
        Ok(guilds) => Ok(guilds.into()),
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}


#[get("/<guild_id>/channels/<channel_id>/roles/<role_id>")]
pub async fn get_permissions_for_role(
    pool: &State<DbPool>, 
    guild_id: i32, 
    channel_id: i32, 
    role_id: i32
) -> Result<Json<ChannelPermissions>, Json<ErrorResponse>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };
    
    use crate::schema::channel_permissions::dsl as cp_dsl;

    let maybe_permissions: Result<ChannelPermissions, _> = cp_dsl::channel_permissions
        .select(ChannelPermissions::as_select())
        .filter(
            cp_dsl::guild_id.eq(guild_id)
                .and(cp_dsl::role_id.eq(role_id))
                .and(cp_dsl::channel_id.eq(channel_id))
        )
        .get_result(&mut conn)
        .await;

    match maybe_permissions {
        Ok(perms) => Ok(perms.into()),
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}
