use std::u8;

use base64::prelude::*;
use chrono::{offset::LocalResult, DateTime, Utc};
use hex::FromHexError;
use ring::{hmac::{self, Tag}, rand::{SecureRandom, SystemRandom}};
use rocket::request::{self, FromRequest, Request};

/*
impl ActiveSession {
    pub fn new(token: &str) -> Option<Self> {
        let parts: Vec<&str> = token.split(":").collect();

        let user_id = String::from_utf8(BASE64_STANDARD.decode(parts.get(0)?).ok()?).ok()?;
        let created_at = String::from_utf8(BASE64_STANDARD.decode(parts.get(1)?).ok()?).ok()?;
        let nonce = String::from_utf8(BASE64_STANDARD.decode(parts.get(2)?).ok()?).ok()?;

        let formatted = format!("{user_id}{created_at}{nonce}");
    }
}

impl<'a, 'r> FromRequest<'a> for ActiveSession {
    type Error = ();

    fn from_request(
        request: &'a Request<'r>
    ) -> request::Outcome<ActiveSession, Self::Error> {
        request
            .cookies()
            .get_private("token")
            .and_then(|cookie| {
                let val = cookie.value();
            })

    }
}
*/

/// The form that is provided when trying to log in as an user
#[derive(Debug, FromForm)]
pub struct LoginForm<'a> {
    pub email: &'a str,
    pub password: &'a str
}

/// Represents a valid and checked token
#[derive(Debug)]
pub struct ActiveSession {
    pub user_id: i32,
    pub generated_at: DateTime<Utc>,
}

impl ActiveSession {
    pub fn new(user_id: i32, generated_at: DateTime<Utc>) -> Self {
        Self {user_id, generated_at}
    }
}

/// A struct used to generate tokens
pub struct TokenHandler {
    rng: SystemRandom,
    key: hmac::Key
}

impl TokenHandler {
    pub fn new() -> Option<Self> {
        let rng = SystemRandom::new();
        let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng).ok()?;

        Some(Self {
            rng,
            key,
        })
    }

    pub fn fill_nonce(&self, data: &mut [u8]) {
        self.rng.fill(data);
    }

    pub fn sign(&self, data: &[u8]) -> String {
        let tag = hmac::sign(&self.key, data);
        hex::encode(tag.as_ref())

    } 

    pub fn verify(&self, data: &[u8], tag: &[u8]) -> Result<bool, FromHexError> {
        let tag = hex::decode(tag)?;

        Ok(hmac::verify(&self.key, data, &tag).is_ok())
    }

}
