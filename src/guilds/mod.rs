use chrono::{DateTime, Utc};
use diesel_async::RunQueryDsl;

pub mod types;
pub use types::*;

mod routes;
pub use routes::routes;

use crate::DbPool;

pub async fn setup(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = pool.get().await?;

    let system_channel = Guild {
        id: 0,
        name: "System".into(),
        owner_id: 0,
        description: "System channel".into(),
        creation_date: Utc::now()
    };

    use crate::schema::guilds::{self, id};

    diesel::insert_into(guilds::table)
        .values(&system_channel)
        .on_conflict(id)
        .do_nothing()
        .execute(&mut connection)
        .await?;

    Ok(())
}