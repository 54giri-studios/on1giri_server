use chrono::Utc;
use diesel::{
    result::Error as DieselError,
    prelude::*
};

use diesel_async::{scoped_futures::ScopedFutureExt, RunQueryDsl};
use rocket::{form::Form, http::{Cookie, CookieJar}, serde::json::Json, time::Duration, State};

use random_word::{self, Lang};

use crate::{AccessLevel, DbPool, ErrorResponse, InitialUserMetadata, JsonResponse, LoginForm, NewUser, TokenHandler, TrimmedNewUser, User, UserMetadata, VerifiedToken};
use crate::schema::{
    users::dsl as u,
    users_metadata::dsl as um 
};


#[post("/register", data = "<credentials>")]
pub async fn register<'a, 'b, 'c>(
    pool: &State<DbPool>,
    token_handler: &State<TokenHandler>,
    cookies: &CookieJar<'a>,
    credentials: Form<LoginForm<'b>>,
) -> JsonResponse<UserMetadata> {

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };



    let access_level = AccessLevel::regular().to_string();

    let new_user = NewUser::new(
        credentials.password, 
        &access_level, 
        credentials.email
    );


    let maybe_meta: Result<UserMetadata, diesel::result::Error> = conn
        .build_transaction()
        .serializable()
        .run(move |conn| async {
            let trimmed: TrimmedNewUser = diesel::insert_into(u::users)
                .values(new_user)
                .returning(TrimmedNewUser::as_returning())
                .get_result(conn)
                .await?;

            let init_meta = InitialUserMetadata::new(
                trimmed.id(),
                random_word::gen(Lang::En),
                "",
                "New account",
            );

            let meta: UserMetadata = diesel::insert_into(um::users_metadata)
                .values(init_meta)
                .returning(UserMetadata::as_returning())
                .get_result(conn)
                .await?;

            Ok(meta)

        }.scope_boxed())
        .await;
    


    match maybe_meta {
        Ok(meta) => {
            let token = VerifiedToken::new(*meta.user_id(), token_handler);
            let token_str = token.raw_string().to_string();

            let cookie = Cookie::build(("token", token_str))
                .secure(true)
                .http_only(true)
                .max_age(Duration::days(7));

            cookies.add_private(cookie);

            Ok(meta.into())
        },
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}
