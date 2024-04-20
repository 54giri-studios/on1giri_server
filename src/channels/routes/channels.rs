use diesel::prelude::*;

use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
use rocket::{serde::json::Json, Error, State};

use crate::{Channel, DbPool, ErrorResponse, NewChannel};

#[get("/<channel_id>")]
pub async fn get_channel(pool: &State<DbPool>, channel_id: i32) -> Result<Json<Channel>, Json<ErrorResponse>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(ErrorResponse::internal_error(err).into())
        }
    };

    use crate::schema::channels::dsl::*;
    let query: Result<Channel, diesel::result::Error> = channels
        .select(Channel::as_select())
        .filter(id.eq(channel_id))
        .get_result(&mut conn)
        .await;

    query
        .map_err(ErrorResponse::from)
        .map(|e| e.into())
        .map_err(|a| a.into())
}

#[post("/create", format = "json", data = "<new_channel>")]
pub async fn post_channel<'a>(
    pool: &State<DbPool>, 
    new_channel: Json<NewChannel<'a>>
) -> Result<Json<Channel<'a>>, Json<ErrorResponse>> {
    let mut conn = match pool.get().await {
        Ok(pool) => pool,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    use crate::schema::channels::dsl as c_dsl;

    let maybe_channel: Result<Channel<'_>, _> = diesel::insert_into(c_dsl::channels)
        .values(new_channel.into_inner())
        .returning(Channel::as_returning())
        .get_result(&mut conn)
        .await;

    match maybe_channel {
        Ok(channel) => Ok(channel.into()),
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}
