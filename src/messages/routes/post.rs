use diesel::IntoSql;
use diesel_async::RunQueryDsl;
use rocket::serde::json::Json;
use rocket::State;

use crate::{DbPool, Message};
use crate::schema::messages;

#[post("/<channel_id>", format = "json", data = "<message>")]
pub async fn post_message<'a>(pool: &State<DbPool>, channel_id: i32, message: Json<Message<'a>>) {
    let message = message.into_inner();

    let mut conn = pool.get().await.unwrap();

    diesel::insert_into(messages::table)
        .values(&message)
        .execute(&mut conn)
        .await;
}