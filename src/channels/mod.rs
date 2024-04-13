use std::borrow::Cow;

use diesel::{deserialize::Queryable, prelude::Insertable};
use diesel_async::RunQueryDsl;

use crate::DbPool;

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelKind {
    Category = 0,
    Text
}

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel<'a> {
    pub id: i32,
    pub guild_id: i32,
    pub name: Cow<'a, str>,
}

pub async fn setup(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {

    let mut conn = pool.get().await.unwrap();

    let system_channel = Channel {
        id: 0,
        guild_id: 0,
        name: "System channel".into()
    };

    use crate::schema::channels::{self, id, guild_id};

    diesel::insert_into(channels::table)
        .values(&system_channel)
        .on_conflict(id)
        .do_nothing()
        .execute(&mut conn)
        .await
        .unwrap();


    Ok(())

}
