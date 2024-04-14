use std::{alloc::System, borrow::Cow, fmt::format};


use chrono::{self, Utc};
use diesel::prelude::*;
use rocket::request::FromRequest;
use serde::Serialize;

use base64::prelude::*;

#[derive(Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::access_levels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccessLevel<'a> {
    level: Cow<'a, str>
}

impl<'a> AccessLevel<'a> {
    pub fn new(level: &'a str) -> Self {
        Self {
            level: level.into()
        }
    }

    pub fn admin() -> Self {
        Self::new("admin")
    }

    pub fn regular() -> Self {
        Self::new("regular")
    }
}


#[derive(Debug, Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User<'a> {
    pub id: i32,
    pub password: Cow<'a, str>,
    pub access_level: Cow<'a, str>,
    pub email: Cow<'a, str>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub password: Cow<'a, str>,
    pub access_level: Cow<'a, str>,
    pub email: Cow<'a, str>
}

#[derive(Debug, Queryable, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserMetadata {
    id: i32,
    username: String,
    discriminator: i16,
    last_check_in: chrono::DateTime<Utc>,
    picture: String,
    account_creation: chrono::DateTime<Utc>,
    description: String
}

#[derive(Debug, Serialize)]
pub struct LoggedUser<'a> {
    pub access_level: Cow<'a, str>,
    pub token: Cow<'a, str>,
    pub metadata: UserMetadata
}
#[derive(FromForm)]
pub struct LoginForm<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

