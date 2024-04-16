use base64::prelude::*;
use chrono::{offset::LocalResult, DateTime, MappedLocalTime, TimeDelta, TimeZone, Utc};
use diesel::sql_types::Date;
use hex::FromHexError;
use ring::{hmac::{self, Tag}, rand::{SecureRandom, SystemRandom}};
use rocket::{http::{ContentType, Status}, request::{self, FromRequest, Request}, serde::json::Json};

use crate::UserMetadata;

/// The form that is provided when trying to log in as an user
#[derive(Debug, FromForm)]
pub struct LoginForm<'a> {
    pub email: &'a str,
    pub password: &'a str
}

/// An unverified raw Token
#[derive(Debug)]
pub struct RawToken<'a> {
    value: &'a str
} 

impl<'a> RawToken<'a> {
    pub fn new(value: &'a str) -> Self {
        Self { value }
    }

    pub fn parse(self) -> Option<ParsedToken<'a>> {
        ParsedToken::from_token(self)
    }

    pub fn value(&self) -> &'a str {
        self.value
    }
}

#[derive(Debug)]
pub struct ParsedToken<'a> {
    user_id: &'a str,
    timestamp: &'a str,
    nonce: &'a str,
    tag: &'a str,
    raw_str: &'a str
}

impl<'a> ParsedToken<'a> {
    fn from_token(raw_token: RawToken<'a>) -> Option<Self> {
        let mut parts = raw_token.value().split(":");

        let user_id = parts.next()?;
        let timestamp = parts.next()?;
        let nonce = parts.next()?;
        let tag = parts.next()?;

        Some(Self {
            user_id,
            timestamp,
            nonce,
            tag,
            raw_str: raw_token.value()
        })
    }

    pub fn verify(self, token_handler: &'a TokenHandler) -> Option<VerifiedToken> {
        VerifiedToken::from_parsed_token(self, token_handler)
    }
}

#[derive(Debug)]
pub struct VerifiedToken  {
    user_id: i32,
    generated_at: DateTime<Utc>,
    raw_str: String
}

impl VerifiedToken {
    pub fn user_id(&self) -> &i32 {
        &self.user_id
    }

    pub fn generated_at(&self) -> &DateTime<Utc> {
        &self.generated_at
    }

    pub fn raw_string<'a>(&'a self) -> &'a str {
        &self.raw_str
    }

    pub fn new(user_id: i32, token_handler: &TokenHandler) -> Self {
        let user_id_str = user_id.to_string();
        let now: DateTime<Utc> = Utc::now();
        let timestamp_str = now.timestamp().to_string();

        let mut nonce = [0_u8; 4];
        token_handler.fill_nonce(&mut nonce);

        let to_encrypt: Vec<u8> = vec![
            user_id_str.as_bytes(),
            timestamp_str.as_bytes(),
            &nonce
        ].concat();


        let signature = token_handler.sign(&to_encrypt);

        // Encode data to make it a bit more compact
        let user_id_encoded = BASE64_URL_SAFE.encode(user_id_str);
        let timestamp_encoded = BASE64_URL_SAFE.encode(timestamp_str);
        let nonce = BASE64_URL_SAFE.encode(hex::encode(nonce));
        let tag = BASE64_URL_SAFE.encode(signature);

        let raw_string = format!("{user_id_encoded}:{timestamp_encoded}:{nonce}:{tag}");

        Self { user_id, generated_at: now, raw_str: raw_string }
    }

    fn from_parsed_token<'a>(parsed_token: ParsedToken<'a>, token_handler: &TokenHandler) -> Option<Self> {
        let decoded_user_id = BASE64_URL_SAFE.decode(parsed_token.user_id).ok()?;
        let decoded_timestamp = BASE64_URL_SAFE.decode(parsed_token.timestamp).ok()?;
        let decoded_nonce = hex::decode(BASE64_URL_SAFE.decode(parsed_token.nonce).ok()?).ok()?;
        let decoded_tag = BASE64_URL_SAFE.decode(parsed_token.tag).ok()?;

        let to_encrypt = [decoded_user_id.clone(), decoded_timestamp.clone(), decoded_nonce].concat();

        let user_id: i32 = String::from_utf8(decoded_user_id)
            .ok()?
            .parse()
            .ok()?;

        let timestamp: i64 = String::from_utf8(decoded_timestamp)
            .ok()?
            .parse()
            .ok()?;

        let is_verified = token_handler.verify(&to_encrypt, &decoded_tag).ok()?;

        if !is_verified {
            return None;
        };

        let generated_at: DateTime<Utc> = match Utc.timestamp_opt(timestamp, 0) {
            MappedLocalTime::Single(time) => time,
            MappedLocalTime::Ambiguous(earliest, latest) => earliest,
            MappedLocalTime::None => return None,
        };

        Some(Self {
            user_id,
            generated_at,
            raw_str: parsed_token.raw_str.to_string()
        })
    }
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

/// A struct used to handle tokens
pub struct TokenHandler {
    rng: SystemRandom,
    key: hmac::Key,
    signature_validity: TimeDelta
}

impl TokenHandler {
    pub fn new(signature_validity: TimeDelta) -> Option<Self> {
        let rng = SystemRandom::new();
        let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng).ok()?;

        Some(Self {
            rng,
            key,
            signature_validity
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

    pub fn signature_validity(&self) -> &TimeDelta {
        &self.signature_validity
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    status: Status,
    data: Option<UserMetadata>
}

impl LoginResponse {
    pub fn new(status: Status, data: Option<UserMetadata>) -> Self {
        Self { status, data }
    }
}