use diesel::{IntoSql, SelectableHelper};
use diesel_async::RunQueryDsl;
use rocket::serde::json::Json;
use rocket::State;

use crate::messages::{InsertableMessage, NewMessage};
use crate::{DbPool, Message};
use crate::schema::messages;

#[post("/<channel_id>", format = "json", data = "<message>")]
pub async fn post_message<'a>(
    pool: &State<DbPool>, 
    channel_id: i32, 
    message: Json<NewMessage<'a>>
) -> Json<Message<'a>>{
    let new_message = message.into_inner();
    let insertable_message = InsertableMessage::new(new_message);

    let mut conn = pool.get().await.unwrap();

    let message = diesel::insert_into(messages::table)
        .values(&insertable_message)
        .returning(Message::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap(); 

    message.into()
}
