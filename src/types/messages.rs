use std::borrow::Cow;

use chrono::{DateTime, Utc};
use diesel::prelude::*;

use rocket::{
    serde::{Deserialize, Serialize},
    tokio::sync::{broadcast::Sender, Mutex},
};
use std::collections::HashMap;

use crate::{Channel, User, UserMetadata};

/// The minimal data that is provided for creating a message
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMessage<'a> {
    channel_id: i32,
    author_id: i32,
    content: Cow<'a, str>,
}

/// Represents a pre-processed [Message] that is ready
/// to be inserted in the database.
/// Can be inserted into [crate::schema::messages]
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableMessage<'a> {
    channel_id: i32,
    author_id: i32,
    content: Cow<'a, str>,
    creation_date: DateTime<Utc>,
}

impl<'a> InsertableMessage<'a> {
    pub fn new(msg: NewMessage<'a>) -> Self {
        Self {
            channel_id: msg.channel_id,
            author_id: msg.author_id,
            content: msg.content,
            creation_date: Utc::now(),
        }
    }

    pub fn channel_id(&self) -> i32 {
        self.channel_id
        
    }

    pub fn author_id(&self) -> i32 {
        self.author_id
    }

}

/// Represents a messaged that was retrieved from the database.
/// Mirrors [crate::schema::messages]
#[derive(Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message<'a> {
    id: i32,
    channel_id: i32,
    author_id: i32,
    content: Cow<'a, str>,
    creation_date: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PopulatedMessage<'a> {
    id: i32,
    channel: Channel,
    author: UserMetadata,
    content: Cow<'a, str>,
    creation_date: DateTime<Utc>,
}

impl<'a> PopulatedMessage<'a> {
    pub fn new(msg: Message<'a>, channel: Channel, author: UserMetadata) -> Self {
        Self {
            id: msg.id,
            channel,
            author,
            content: msg.content,
            creation_date: msg.creation_date
        }
    }
}

// the different types of messages that a client could
// send down the channel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromFormField)]
#[serde(crate = "rocket::serde")]
pub enum MessageType {
    CONNECT,
    SEND,
    QUIT,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChannelMessage {
    pub id: i32,
    pub channel_id: i32,
    pub message_type: MessageType,
    pub content: String,
    pub author_id: i32,
    pub creation_date: DateTime<Utc>,
}

impl ChannelMessage {
    pub fn new(msg: &Message) -> Self {
        Self {
            id: msg.id,
            channel_id: msg.channel_id,
            author_id: msg.author_id,
            content: msg.content.to_string(),
            message_type: MessageType::SEND,
            creation_date: msg.creation_date,
        }
    }
}

pub struct AppState {
    // will contain the room ids with the Sender end of the
    // broadcast sockets
    // each connections between a client and the server is in here
    pub clients: Mutex<HashMap<i32, Sender<ChannelMessage>>>,
}
