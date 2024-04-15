use base64::prelude::*;

use chrono::{DateTime, MappedLocalTime, TimeZone, Utc};
use diesel::prelude::*;

use crate::{login::TokenHandler, ActiveSession, DbPool, LoginForm, User, UserMetadata};
use diesel_async::RunQueryDsl;
use rocket::Response;
use rocket::{
    form::Form,
    http::{ContentType, Cookie, CookieJar, SameSite, Status},
    response::status,
    serde::json::Json,
    State,
};

fn check_token(
    token_handler: &State<TokenHandler>,
    cookies: &CookieJar<'_>,
) -> Option<ActiveSession> {
    let token = cookies.get_private("token")?;
    let mut parts = token.value().split(':');

    let user_id = BASE64_URL_SAFE.decode(parts.next()?).ok()?;
    let timestamp = BASE64_URL_SAFE.decode(parts.next()?).ok()?;
    let nonce = BASE64_URL_SAFE.decode(parts.next()?).ok()?;
    let tag = parts.next()?;

    let to_encrypt = [user_id.clone(), timestamp.clone(), nonce].concat();

    let is_verified = token_handler
        .verify(to_encrypt.as_ref(), tag.as_ref())
        .ok()?;

    if is_verified {
        let user_id: i32 = String::from_utf8(user_id).ok()?.parse().ok()?;
        let timestamp: i64 = String::from_utf8(timestamp).ok()?.parse().ok()?;

        let generated_at: DateTime<Utc> = match Utc.timestamp_opt(timestamp, 0) {
            MappedLocalTime::Single(time) => time,
            // Always assume the worse
            MappedLocalTime::Ambiguous(earliest, latest) => earliest,
            MappedLocalTime::None => return None,
        };

        Some(ActiveSession::new(user_id, generated_at))
    } else {
        None
    }
}

fn generate_token(token_handler: &State<TokenHandler>, user_id: i32) -> String {
    let user_id = user_id.to_string();
    let timestamp = Utc::now().timestamp().to_string();

    // Might be overkill, but the norm is to 
    // use nonces as big as the block size of the underlying hash
    // function, in our case, HMAC uses 64 bytes
    let mut nonce = [0_u8; 32];
    token_handler.fill_nonce(&mut nonce);

    let to_encrypt: Vec<&[u8]> = vec![
        user_id.as_bytes(),
        timestamp.as_bytes(),
        &nonce
    ];
    let to_encrypt = to_encrypt.concat();

    let signature = token_handler.sign(&to_encrypt);

    // Encode data for obfuscation purposes
    let user_id = BASE64_URL_SAFE.encode(user_id);
    let timestamp = BASE64_URL_SAFE.encode(timestamp);
    let nonce = BASE64_URL_SAFE.encode(hex::encode(nonce));

    format!("{user_id}:{timestamp}:{nonce}:{signature}")
}

#[post("/", data = "<credentials>")]
pub async fn login<'a>(
    pool: &State<DbPool>,
    token_handler: &State<TokenHandler>,
    cookies: &CookieJar<'a>,
    credentials: Option<Form<LoginForm<'a>>>,
) -> (Status, Option<Json<UserMetadata>>) {
    use crate::schema::users::dsl as u_dsl;
    use crate::schema::users_metadata::dsl as m_dsl;

    let mut conn = pool.get().await.unwrap();

    // User has a valid token, just check whether the user_id in the token
    // still refers to an actual user
    if let Some(active_session) = check_token(token_handler, cookies) {
        let query: Result<UserMetadata, _> = m_dsl::users_metadata
            .select(UserMetadata::as_select())
            .filter(m_dsl::id.eq(active_session.user_id))
            .get_result(&mut conn)
            .await;

        let Ok(user_meta) = query else {
            return (Status::NotFound, None);
        };

        return (Status::Ok, Some(user_meta.into()));
    };

    // User has no token and didn't provide credentials, reject
    let Some(credentials) = credentials else {
        return (Status::Unauthorized, None);
    };

    let query: Result<UserMetadata, _> = u_dsl::users
        .inner_join(m_dsl::users_metadata)
        .select(UserMetadata::as_select())
        .filter(
            u_dsl::email
                .eq(credentials.email)
                .and(u_dsl::password.eq(credentials.password)),
        )
        .get_result(&mut conn)
        .await;

    // Username / Password didn't match
    let Ok(user_metadata) = query else {
        return (Status::Unauthorized, None);
    };

    // Time to construct a new cookie

    let token_val = generate_token(token_handler, user_metadata.id);
    cookies.add_private(Cookie::new("token", token_val));

    (Status::Ok, Some(user_metadata.into()))
}
