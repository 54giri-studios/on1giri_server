use chrono::{self, Utc};
use serde::Serialize;

#[derive(Debug)]
pub struct User {
    id: i32,
    password: String,
    user_type: i32,
    email: String
}

#[derive(Debug)]
pub struct UserMetadata {
    id: i32,
    username: String,
    discriminator: i16,
    last_check_in: chrono::DateTime<Utc>,
    picture: String,
    account_creation: chrono::DateTime<Utc>,
    description: String
}


