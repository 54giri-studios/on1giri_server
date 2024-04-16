use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use rocket::serde::json::Json;
use rocket::State;

use crate::{DbPool, Message};

#[get("/<channel_id>/<message_id>")]
pub async fn get_message<'a>(
    pool: &State<DbPool>, 
    channel_id: i32, 
    message_id: i32
) -> Json<Message<'a>>{
    let mut conn = pool.get().await.unwrap();

    use crate::schema::messages;

    let message = messages::table
        .select(Message::as_select())
        .filter(messages::channel_id.eq(channel_id).and(messages::id.eq(message_id)))
        .get_result(&mut conn)
        .await
        .unwrap();

    message.into()
}

#[get("/<channel_id>/history/<num_messages>")]
pub async fn get_message_history(channel_id: i32, num_messages: usize) {
    todo!()
}