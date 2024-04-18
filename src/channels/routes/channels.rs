use diesel::prelude::*;

use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{Channel, DbPool, ErrorResponse};

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
