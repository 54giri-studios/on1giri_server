use std::borrow::Cow;


use chrono::Utc;
use diesel::prelude::*;
use serde::Serialize;



#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub password: Cow<'a, str>,
    pub access_level: Cow<'a, str>,
    pub email: Cow<'a, str>
}

#[derive(Debug, AsChangeset, Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    password: String,
    access_level: String,
    email: String,
}

impl User {
    pub fn new(id: i32, password: String, access_level: String, email: String) -> Self {
        Self { id, password, access_level, email }
    }
}
