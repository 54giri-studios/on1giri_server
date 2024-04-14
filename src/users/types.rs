use std::{borrow::Cow, slice::Iter};


use chrono::{self, Utc};
use diesel::prelude::*;
use ring::rand::SystemRandom;
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
    pub token: Cow<'a, str>
}

impl<'a> User<'a> {
    pub fn new(
        id: i32,
        email: &'a str, 
        password: &'a str, 
        access_level: &'a str,
    ) -> Self {

        let timestamp: String = chrono::Utc::now()
            .timestamp()
            .to_string();

        let secure: u32 = 0;
        let mut token = String::new();

        token += &BASE64_STANDARD.encode(timestamp);
        token += &BASE64_STANDARD.encode(id.to_string());
        token += &BASE64_STANDARD.encode(secure.to_string());
        Self {
            id,
            email: email.into(),
            password: password.into(),
            access_level: access_level.into(),
            token: token.into()
        }
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub password: Cow<'a, str>,
    pub access_level: Cow<'a, str>,
    pub email: Cow<'a, str>
}

#[derive(Debug, Queryable, Insertable, Selectable)]
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

#[derive(FromForm)]
pub struct LoginForm<'a> {
    pub email: &'a str,
    pub password: &'a str,
}
