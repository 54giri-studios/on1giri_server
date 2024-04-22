use chrono::{DateTime, Utc};
use diesel::prelude::*;

use diesel_async::RunQueryDsl;
use ring::rsa::PublicKeyComponents;
use rocket::{serde::json::Json, State};

use crate::{Channel, DbPool, ErrorResponse, HistoryConfig, Message, PopulatedMessage, User, UserMetadata};

/// Retrieves a channel's most recent messages
#[post("/<channel_id>/messages/history", data = "<history_config>", format = "json")]
pub async fn get_channel_history(pool: &State<DbPool>, channel_id: i32, history_config: Json<HistoryConfig>) -> Result<Json<Vec<PopulatedMessage>>, Json<ErrorResponse>> {
    let mut conn = pool.get().await.unwrap();

    use crate::schema::{
        messages::dsl as m_dsl,
        channels::dsl as c_dsl,
        users_metadata::dsl as um_dsl
    };

    let history_config = history_config.into_inner();

    let mut boxed_select = m_dsl::messages
        .filter(m_dsl::channel_id.eq(channel_id))
        .inner_join(um_dsl::users_metadata.on(um_dsl::id.eq(m_dsl::author_id)))
        .inner_join(c_dsl::channels)
        .into_boxed();

    if let Some(before) = history_config.before {
        boxed_select = boxed_select.filter(m_dsl::creation_date.le(before));
    };

    if let Some(after) = history_config.after {
        boxed_select = boxed_select.filter(m_dsl::creation_date.ge(after));
    }

    boxed_select = boxed_select.order_by(m_dsl::creation_date.desc());

    if let Some(limit) = history_config.limit {
        boxed_select = boxed_select.limit(limit as i64);
    }

    let maybe_messages: Result<Vec<(Message, UserMetadata, Channel)>, _> = boxed_select
        .get_results(&mut conn)
        .await;

    match maybe_messages {
        Ok(messages) => {
            let mapped: Vec<PopulatedMessage> = messages
                .into_iter()
                .map(|(m, um, c)| PopulatedMessage::new(m, c, um))
                .collect();

            Ok(mapped.into())
        },
        Err(err) => {
            Err(ErrorResponse::from(err).into())
        }
    }
}

#[get("/<channel_id>/messages/<message_id>")]
pub async fn get_message(
    pool: &State<DbPool>, 
    channel_id: i32, 
    message_id: i32
) -> Result<Json<PopulatedMessage>, Json<ErrorResponse>> {

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    use crate::schema::{
        messages::dsl as m_dsl,
        channels::dsl as c_dsl,
        users_metadata::dsl as um_dsl
    };

    let pieces: Result<(Message, UserMetadata, Channel), _> = m_dsl::messages
        .filter(
            m_dsl::id.eq(message_id)
                .and(
                    m_dsl::channel_id.eq(channel_id)
                )
        )
        .inner_join(um_dsl::users_metadata.on(um_dsl::id.eq(m_dsl::author_id)))
        .inner_join(c_dsl::channels)
        .get_result(&mut conn)
        .await;

    match pieces {
        Ok((msg, meta, chan)) => Ok(PopulatedMessage::new(msg, chan, meta).into()),
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}