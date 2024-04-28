use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use rocket::{serde::json::Json, State};

use crate::{Channel, DbPool, ErrorResponse, JsonResponse};
use crate::schema::channels::dsl as c;

#[get("/<guild_id>/channels")]
pub async fn get_guild_channels(
    pool: &State<DbPool>,
    guild_id: i32,
) -> JsonResponse<Vec<Channel>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    let maybe_channels: Result<Vec<Channel>, _> = c::channels
        .filter(c::guild_id.eq(guild_id))
        .get_results(&mut conn)
        .await;

    let channels: Vec<Channel> = match maybe_channels {
        Ok(c) => c,
        Err(err) => return Err(ErrorResponse::from(err).into())
    };

    Ok(channels.into())
}
