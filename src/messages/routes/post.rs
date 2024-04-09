use rocket::serde::json::Json;

use crate::Message;

#[post("/<channel_id>", format = "json", data = "<message>")]
pub async fn post_message(channel_id: i32, message: Json<Message>) {
    let message = message.into_inner();
    println!("{:#?}", message);
}