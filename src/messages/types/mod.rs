use std::borrow::Cow;

use chrono::{DateTime, Utc};
use diesel::{deserialize::{FromSqlRow, Queryable}, prelude::Insertable, Selectable};
use rocket::{data::{self, FromData}, Request, Data};

use crate::User;

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
    creation_date: DateTime<Utc>
}

impl<'a> InsertableMessage<'a> {
    pub fn new(msg: NewMessage<'a>) -> Self {
        Self {
            channel_id: msg.channel_id,
            author_id: msg.author_id,
            content: msg.content,
            creation_date: Utc::now()
        }
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
    creation_date: DateTime<Utc>
}

