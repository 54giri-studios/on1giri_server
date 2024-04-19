//use crate::login::routes::post::check_token;
use crate::login::TokenHandler;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use rocket::http::CookieJar;
use rocket::serde::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::response::Responder;
use rocket::State;
use crate::messages::Message;

use crate::messages::{InsertableMessage, NewMessage};
use crate::schema::messages;
use crate::{DbPool, ChannelMessage, AppState};

#[derive(Debug, Responder)]
enum ApiResponse {
    #[response(status = 200, content_type = "json")]
    SUCCEEDED(Value),
    #[response(status = 401, content_type = "json")]
    FAILED(Value),
}

#[post("/", format = "json", data = "<message>")]
pub async fn post_message<'a>(
    pool: &State<DbPool>,
    token_handler: &State<TokenHandler>,
    chat_sessions: &State<AppState>,
    cookies: &CookieJar<'a>,
    message: Json<NewMessage<'a>>,
) -> ApiResponse {
    // token verification
    let mut conn = pool.get().await.unwrap();

    // let Some(verified_token) = check_token(token_handler, cookies) else {
    //     // create new insertable message that contains error
    //     return ApiResponse::FAILED(
    //         json::json!({ "error": "Token expired, should login back" }));
    //     
    // };

    // saving to the database
    let new_message = message.into_inner();
    let insertable_message = InsertableMessage::new(new_message);

    let message = diesel::insert_into(messages::table)
        .values(&insertable_message)
        .returning(Message::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    // Sending the message down the channel for other users to receive
    let channel_msg = ChannelMessage::new(insertable_message);
    let mut chat_sessions = chat_sessions.clients.lock().await;

    let message = json::json!(message);

    if let Some(queue) = chat_sessions.get(&channel_msg.channel_id) {
        if let Err(_) = queue.send(channel_msg.clone()) {
            chat_sessions.remove(&channel_msg.channel_id);
        }
    }
    

    ApiResponse::SUCCEEDED(message)
}
