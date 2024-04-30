use std::borrow::Cow;

use chrono::{DateTime, Utc};
use diesel::{prelude::*, sql_types::Date};

use crate::{Channel, Role, UserMetadata};

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
    owner_id: i32,
    /// A short description telling what kind of activities
    /// is done in this guild
    pub description: String,
    /// When it was created for the first time
    pub creation_date: DateTime<Utc>
}

#[derive(Debug, AsChangeset, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::guilds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchGuild<'a> {
    name: Option<Cow<'a, str>>,
    owner_id: Option<i32>,
    description: Option<Cow<'a, str>>,
}

impl Guild {
    pub fn new(id: i32, name: String, owner_id: i32, description: String, creation_date: DateTime<Utc>) -> Self {
        Self { id, name, owner_id, description, creation_date }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn owner_id(&self) -> i32 {
        self.owner_id
    }

}

#[derive(Debug, Serialize)]
pub struct PopulatedGuild {
    id: i32,
    name: String,
    owner: UserMetadata,
    description: String,
    creation_date: DateTime<Utc>,
    roles: Vec<Role>,
    channels: Vec<Channel>
}

impl PopulatedGuild {
    pub fn new(
        guild: Guild, 
        owner: UserMetadata, 
        roles: Vec<Role>, 
        channels: Vec<Channel>
    ) -> Self {
        Self {
            id: guild.id,
            name: guild.name,
            owner,
            description: guild.description,
            creation_date: guild.creation_date,
            roles,
            channels
        }
    }
}