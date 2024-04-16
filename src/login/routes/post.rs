use chrono::Utc;
use diesel::prelude::*;
use rocket::time::Duration;

use crate::{LoginResponse, RawToken, VerifiedToken};
use crate::{login::TokenHandler, DbPool, LoginForm, UserMetadata};
use diesel_async::RunQueryDsl;


use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
    State,
};

fn check_token<'a>(
    token_handler: &'a State<TokenHandler>,
    cookies: &CookieJar<'a>,
) -> Option<VerifiedToken> {

    let token = cookies.get_private("token")?;

    let token = RawToken::new(token.value());
    let token = token.parse()?;

    println!("{:#?}", token);

    let verified = token.verify(&token_handler)?;

    println!("{:#?}", verified);


    if Utc::now() - verified.generated_at() < *token_handler.signature_validity() {
        Some(verified)
    } else {
        None
    }
}

#[post("/", data = "<credentials>")]
pub async fn login<'a, 'b>(
    pool: &State<DbPool>,
    token_handler: &State<TokenHandler>,
    cookies: &CookieJar<'a>,
    credentials: Option<Form<LoginForm<'b>>>
) -> Json<LoginResponse> {

    let mut conn = pool.get().await.unwrap();

    use crate::schema::users_metadata::dsl as m_dsl;
    use crate::schema::users::dsl as u_dsl;

    if let Some(verified_token) = check_token(token_handler, cookies) {

        let maybe_meta: Result<UserMetadata, _> = m_dsl::users_metadata
            .select(UserMetadata::as_select())
            .filter(m_dsl::id.eq(verified_token.user_id()))
            .get_result(&mut conn)
            .await;

        if let Ok(meta) = maybe_meta {
            return LoginResponse::new(Status::Ok, Some(meta)).into();
        };
    };


    let Some(credentials) = credentials else {
        return LoginResponse::new(Status::BadRequest, None).into();
    };

    let query: Result<UserMetadata, _> = u_dsl::users
        .inner_join(m_dsl::users_metadata)
        .select(UserMetadata::as_select())
        .filter(
            u_dsl::email.eq(credentials.email)
                .and(u_dsl::password.eq(credentials.password)),
        )
        .get_result(&mut conn)
        .await;

    match query {
        Err(_) => {
            LoginResponse::new(Status::Forbidden, None).into()
        },
        Ok(meta) => {
            let token = VerifiedToken::new(*meta.user_id(), token_handler);
            let token_str = token.raw_string().to_string();

            let cookie = Cookie::build(("token", token_str))
                .secure(true)
                .http_only(true)
                .max_age(Duration::days(7));

            cookies.add_private(cookie);

            LoginResponse::new(Status::Ok, Some(meta)).into()
        }
    }
}
