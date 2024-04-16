use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::DbPool;

pub mod types;
pub use types::*; 

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

    let system_channel = Channel {
        id: 0,
        guild_id: 0,
        name: "System channel".into(),
        kind: "system".into()
    };

    use crate::schema::channels;

    diesel::insert_into(channels::table)
        .values(&system_channel)
        .on_conflict(channels::id)
        .do_update()
        .set((
            channels::name.eq(&system_channel.name),
            channels::kind.eq(&system_channel.kind)
        ))
        .execute(&mut conn)
        .await
        .unwrap();

    Ok(())
}
