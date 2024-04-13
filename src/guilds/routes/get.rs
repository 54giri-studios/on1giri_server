use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State}; 
use crate::{Channel, DbPool};

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