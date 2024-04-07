use std::{borrow::Cow, slice::Iter};


use chrono::{self, Utc};
use diesel::prelude::*;
use serde::Serialize;

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


#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User<'a> {
    pub id: i32,
    pub password: Cow<'a, str>,
    pub access_level: Cow<'a, str>,
    pub email: Cow<'a, str>
}

#[derive(Debug, Queryable, Insertable)]
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

