use std::borrow::Cow;


use chrono::Utc;
use diesel::prelude::*;
use serde::Serialize;



#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    password: Cow<'a, str>,
    access_level: Cow<'a, str>,
    email: Cow<'a, str>
}

impl<'a> NewUser<'a> {
    pub fn new(password: &'a str, access_level: &'a str, email: &'a str) -> Self {
        Self { 
            password: password.into() , 
            access_level: access_level.into(), 
            email : email.into()
        }
    }
}


#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TrimmedNewUser<'a> {
    id: i32,
    access_level: Cow<'a, str>,
    email: Cow<'a, str>
}

impl<'a> TrimmedNewUser<'a> {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn access_level(&'a self) -> &'a str {
        &self.access_level
    }

    pub fn email(&'a self) -> &'a str {
        &self.email
    }
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
