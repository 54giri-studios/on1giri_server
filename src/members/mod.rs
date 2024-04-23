use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{DbPool, Member};

pub async fn setup(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get().await?;

    use crate::schema::{
        guilds::dsl as g_dsl,
        members::dsl as m_dsl,
    };

    let guild_ids: Vec<i32> = g_dsl::guilds
        .select(g_dsl::id)
        .get_results(&mut conn)
        .await?;

    for guild_id in guild_ids {

        let overlord_member = Member::new(0, guild_id);

        diesel::insert_into(m_dsl::members)
            .values(&overlord_member)
            .on_conflict((m_dsl::user_id, m_dsl::guild_id))
            .do_update()
            .set(&overlord_member)
            .execute(&mut conn)
            .await?;
    }
    Ok(())
}
