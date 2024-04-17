use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{Channel, DbPool};

#[get("/<channel_id>")]
pub async fn get_channel(pool: &State<DbPool>, channel_id: i32) -> Json<Channel> {
    let mut conn = pool.get().await.unwrap();

    use crate::schema::channels::dsl::*;
    let channel = channels
        .select(Channel::as_select())
        .filter(id.eq(channel_id))
        .get_result(&mut conn)
        .await
        .unwrap();

    channel.into()
}

