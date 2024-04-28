use diesel::prelude::*;
use std::borrow::Cow;
use chrono::{DateTime, Utc};

use crate::{Guild, Role};

/// A channel draft provided by the user to create a new one
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewChannel<'a> {
    guild_id: i32,
    name: Cow<'a, str>,
    kind: Cow<'a, str>
}

/// Represents a generic channel.
/// Mirrors [crate::schema::channels]
#[derive(Debug, AsChangeset, Serialize, Insertable, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel {
    /// It's globally unique id
    pub id: i32,
    /// The guild's id that it belongs to
    /// Must refer to an actual [crate::Guild]
    guild_id: i32,
    /// It's display name
    pub name: String,
    /// The kind of the channel
    /// Must refer to an actual [crate::ChannelKind]
    pub kind: String
}

impl Channel {
    pub fn new(id: i32, guild_id: i32, name: String, kind: String) -> Self {
        Self { id, guild_id, name, kind }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn guild_id(&self) -> i32 {
        self.guild_id
    }
}

/// An enum like defining a [Channel]'s kind.
/// Mirrors [crate::schema::channel_kinds]
#[derive(Debug, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::channel_kinds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelKind {
    /// The kind of the channel
    /// Must be constructed through it's methods
    kind: String
}

impl ToString for ChannelKind {
    fn to_string(&self) -> String {
        self.kind.clone()
    }

}

impl ChannelKind {
    fn new(kind: String) -> Self {
        Self { kind }
    }

    /// A text channel: Users can write into it
    pub fn text() -> Self {
        Self::new("text".into())
    }

    /// A category channel: Used to group channels together
    pub fn category() -> Self {
        Self::new("category".into())
    }

    /// A voice channel: Can be connected to
    pub fn voice() -> Self {
        Self::new("voice".into())
    }

    /// A system channel: used by the server to send data
    pub fn system() -> Self {
        Self::new("system".into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryConfig {
    pub limit: Option<i32>,
    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>
}

