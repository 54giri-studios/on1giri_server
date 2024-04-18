use chrono::{DateTime, Utc};
use diesel::prelude::*;

use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{DbPool, HistoryConfig, Message};

#[post("/<channel_id>/messages/history", data = "<history_config>")]
pub async fn get_channel_history(pool: &State<DbPool>, channel_id: i32, history_config: Json<HistoryConfig>) -> Json<Vec<Message>>{
    let mut conn = pool.get().await.unwrap();

    use crate::schema::messages::dsl as m_dsl;

    let history_config = history_config.into_inner();

    let messages: Vec<Message> = m_dsl::messages
        .select(Message::as_select())
        .limit(history_config.limit.unwrap_or(100).max(0).min(100) as i64)
        .filter(
            m_dsl::channel_id.eq(channel_id)
                .and(
                    m_dsl::creation_date.between(
                        history_config.before.unwrap_or(DateTime::<Utc>::MIN_UTC),
                        history_config.after.unwrap_or(DateTime::<Utc>::MAX_UTC)
                )
            )
        )
        .get_results(&mut conn)
        .await
        .unwrap_or(Vec::new());

    messages.into()
}

/*
pub async fn get_channel(pool: &State<DbPool>, channel_id: i32) -> Json<Message> {
}
*/
