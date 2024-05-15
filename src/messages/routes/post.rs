use crate::{Channel, ErrorResponse, Member};
//use crate::login::routes::post::check_token;
use crate::TokenHandler;

use diesel::prelude::*;
use diesel_async::{RunQueryDsl, scoped_futures::ScopedFutureExt};
use diesel::result::Error as DieselError;
use rocket::http::{CookieJar, Status};
use rocket::serde::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::response::Responder;
use rocket::State;

use crate::{ChannelKind, InsertableMessage, NewMessage, Message, Guild};
use crate::schema::{
    messages::dsl as msg,
    members::dsl as mem,
    channels::dsl as c,
    guilds::dsl as g,
};
use crate::{DbPool, ChannelMessage, AppState};

#[derive(Debug, Responder)]
pub enum ApiResponse {
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
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(err) => return ApiResponse::FAILED(
            json::json!(ErrorResponse::internal_error(err))
        ),
    };

    // let Some(verified_token) = check_token(token_handler, cookies) else {
    //     // create new insertable message that contains error
    //     return ApiResponse::FAILED(
    //         json::json!({ "error": "Token expired, should login back" }));
    //     
    // };

    // saving to the database
    let new_message = message.into_inner();
    let insertable_message = InsertableMessage::new(new_message);

    let maybe_channel: Result<Channel, _> = c::channels
        .filter(c::id.eq(insertable_message.channel_id()))
        .get_result(&mut conn)
        .await;

    let channel: Channel = match maybe_channel {
        Ok(c) => c,
        Err(err) => {
            match err {
                DieselError::NotFound => {
                    return ApiResponse::FAILED(
                        json::json!(ErrorResponse::new(
                            Status::BadRequest,
                            format!("Channel {} does not exist", insertable_message.channel_id())
                        ))
                    );
                },
                db_err => {
                    return ApiResponse::FAILED(
                        json::json!(ErrorResponse::internal_error(db_err))
                    )
                }
            }
        }
    };

    if channel.kind != ChannelKind::text().to_string() {
        return ApiResponse::FAILED(json::json!(
            ErrorResponse::new(
                Status::BadRequest,
                format!("Chanel {} is not a text channel", channel.id())
            )
        ));
    }

    let maybe_member: Result<Member, _> = mem::members
            .filter(mem::user_id.eq(insertable_message.author_id()))
            .filter(mem::guild_id.eq(channel.guild_id()))// TODO !
            .get_result(&mut conn)
            .await;

    let member: Member = match maybe_member {
        Ok(m) => m,
        Err(err) => return ApiResponse::FAILED(
            json::json!(
                ErrorResponse::new(
                    Status::Forbidden,
                    format!(
                        "Member {} does not belong to guild {}", 
                        insertable_message.author_id(),
                        channel.guild_id(),
                    )
                )
            )
        ),
    };

    let maybe_message: Result<Message, _> = diesel::insert_into(msg::messages)
        .values(&insertable_message)
        .returning(Message::as_returning())
        .get_result(&mut conn)
        .await;

    let message = match maybe_message {
        Ok(m) => m,
        Err(err) => return ApiResponse::FAILED(json::json!(
            ErrorResponse::internal_error(err)
        ))
    };

    // Sending the message down the channel for other users to receive
    let channel_msg = ChannelMessage::new(&message);
    let mut chat_sessions = chat_sessions.clients.lock().await;

    let message = json::json!(message);

    if let Some(queue) = chat_sessions.get(&channel_msg.channel_id) {
        if let Err(_) = queue.send(channel_msg.clone()) {
            chat_sessions.remove(&channel_msg.channel_id);
        }
    }
    

    ApiResponse::SUCCEEDED(message)
}
