use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::DbPool;
use crate::{Channel, ChannelKind};

mod routes;
pub use routes::*;

pub async fn setup(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {

    let mut conn = pool.get().await.unwrap();

    use crate::schema::channel_kinds;

    let kinds = [
        ChannelKind::text(), 
        ChannelKind::category(),
        ChannelKind::voice(),
        ChannelKind::system()
    ];

    for kind in kinds {
        diesel::insert_into(channel_kinds::table)
            .values(&kind)
            .on_conflict(channel_kinds::kind)
            .do_nothing()
            .execute(&mut conn)
            .await
            .unwrap();
    }

    Ok(())
}
