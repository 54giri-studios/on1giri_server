use std::borrow::Cow;

use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Newguild {
    name: String,
    owner_id: i32,
    description: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::guilds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableGuild {
    name: String,
    owner_id: i32,
    description: String,
    creation_date: DateTime<Utc>
}

impl InsertableGuild {
    pub fn new(new_guild: Newguild) -> Self {
        Self {
            name: new_guild.name,
            owner_id: new_guild.owner_id,
            description: new_guild.description,
            creation_date: Utc::now()
        }
    }
}

/// Represents a guild
/// Mirrors [crate::schema::guilds]
#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset, Insertable, Queryable, Selectable, QueryableByName, PartialEq)]
#[diesel(table_name = crate::schema::guilds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guild {
    /// The guild's unique id
    pub id: i32,
    /// It's name, might go through validation
    pub name: String,
    /// Its owner's id.
    /// Must refer to an actual [crate::User]
    pub owner_id: i32,
    /// A short description telling what kind of activities
    /// is done in this guild
    pub description: String,
    /// When it was created for the first time
    pub creation_date: DateTime<Utc>
}

impl Guild {
    pub fn new(id: i32, name: String, owner_id: i32, description: String, creation_date: DateTime<Utc>) -> Self {
        Self { id, name, owner_id, description, creation_date }
    }
}
