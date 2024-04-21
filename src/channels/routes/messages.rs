use chrono::{DateTime, Utc};
use diesel::prelude::*;

use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{DbPool, ErrorResponse, HistoryConfig, Message};

/// Retrieves a channel's most recent messages
#[post("/<channel_id>/messages/history", data = "<history_config>", format = "json")]
pub async fn get_channel_history(pool: &State<DbPool>, channel_id: i32, history_config: Json<HistoryConfig>) -> Result<Json<Vec<Message>>, Json<ErrorResponse>> {
    let mut conn = pool.get().await.unwrap();

    use crate::schema::messages::dsl as m_dsl;

    let history_config = history_config.into_inner();

    let mut boxed_select = m_dsl::messages.into_boxed();
    boxed_select = boxed_select.filter(m_dsl::channel_id.eq(channel_id));

    if let Some(before) = history_config.before {
        boxed_select = boxed_select.filter(m_dsl::creation_date.le(before));
    };

    if let Some(after) = history_config.after {
        boxed_select = boxed_select.filter(m_dsl::creation_date.ge(after));
    }

    if let Some(limit) = history_config.limit {
        boxed_select = boxed_select.limit(limit as i64);
    }

    let maybe_messages: Result<Vec<Message>, _> = boxed_select
        .get_results(&mut conn)
        .await;

    match maybe_messages {
        Ok(messages) => Ok(messages.into()),
        Err(err) => {
            Err(ErrorResponse::from(err).into())
        }
    }
}