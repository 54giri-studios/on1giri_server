use std::borrow::Cow;

use chrono::{DateTime, Utc};
use diesel::{deserialize::Queryable, prelude::Insertable};

/// Represents a guild
/// Mirrors [crate::schema::guilds]
#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = crate::schema::guilds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guild<'a> {
    /// The guild's unique id
    pub id: i32,
    /// It's name, might go through validation
    pub name: Cow<'a, str>,
    /// Its owner's id.
    /// Must refer to an actual [crate::User]
    pub owner_id: i32,
    /// A short description telling what kind of activities
    /// is done in this guild
    pub description: Cow<'a, str>,
    /// When it was created for the first time
    pub creation_date: DateTime<Utc>
}
