use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State}; 
use crate::{Channel, DbPool, ErrorResponse, Guild};

#[get("/<guild_id>/channels")]
pub async fn get_channels(pool: &State<DbPool>, guild_id: i32) -> Json<Vec<Channel>> {
    let mut conn = pool.get().await.unwrap();

    use crate::schema::channels::dsl;
    let channels = dsl::channels
        .select(Channel::as_select())
        .filter(dsl::guild_id.eq(guild_id))
        .get_results(&mut conn)
        .await
        .unwrap();

    channels.into()
}

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
