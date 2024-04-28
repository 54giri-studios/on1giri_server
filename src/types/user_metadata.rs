use std::borrow::Cow;

use diesel::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Queryable, Insertable, Selectable, Serialize, AsChangeset, PartialEq)]
#[diesel(table_name = crate::schema::users_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserMetadata {
    pub id: i32,
    pub username: String,
    pub discriminator: i32,
    pub last_check_in: DateTime<Utc>,
    pub picture: String,
    pub account_creation: DateTime<Utc>,
    pub description: String
}

impl UserMetadata {
    pub fn user_id(&self) -> &i32 {
        &self.id
    }

    pub fn new(
        id: i32,
        username: String,
        discriminator: i32,
        last_check_in: DateTime<Utc>,
        picture: String,
        account_creation: DateTime<Utc>,
        description: String,
    ) -> Self {
        Self { id, username, discriminator, last_check_in, picture, account_creation, description }
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InitialUserMetadata<'a> {
    id: i32,
    username: Cow<'a, str>,
    picture: Cow<'a, str>,
    description: Cow<'a, str>,
}

impl<'a> InitialUserMetadata<'a> {
    pub fn new(id: i32, username: &'a str, picture: &'a str, description: &'a str) -> Self {
        Self { 
            id, 
            username: username.into(), 
            picture: picture.into(), 
            description: description.into(), 
        }
    }
}

#[derive(Debug, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = crate::schema::users_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchUserMetadata<'a> {
    username: Option<Cow<'a, str>>,
    discriminator: Option<i32>,
    picture: Option<Cow<'a, str>>,
    description: Option<Cow<'a, str>>
}
