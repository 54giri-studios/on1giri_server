use core::fmt;
use std::fmt::format;

use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection, AsyncPgConnection};

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

use crate::{DbPool, Message};

#[get("/<channel_id>/<message_id>")]
pub async fn get_message(pool: &State<DbPool>,channel_id: i32, message_id: i32) {

    let mut conn = pool.get().await.unwrap();

    use crate::schema::messages;

    // let m = messages::table.load::<Message>(&mut conn).await;


    todo!()

}

#[get("/<channel_id>/history/<num_messages>")]
pub async fn get_message_history(channel_id: i32, num_messages: usize) {
    todo!()
}