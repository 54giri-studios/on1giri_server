use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{form::Form, serde::json::Json, State};

use crate::{schema::users, users::{LoginForm, NewUser}, DbPool, User, UserMetadata};

#[post("/create", format = "json", data = "<new_user>")]
pub async fn user_create<'a>(pool: &State<DbPool>, new_user: Json<NewUser<'a>>) -> Json<User<'a>> {
    let mut conn = pool.get().await.unwrap();

    let new_user = new_user.into_inner();

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    user.into()
}

#[post("/login", format = "json", data = "<credentials>")]
pub async fn user_login<'a>(pool: &State<DbPool>, credentials: Form<LoginForm<'a>>) -> Json<User<'a>> {
    let mut conn = pool.get().await.unwrap();
    let login = credentials.into_inner();


    use crate::schema::users::dsl as user_dsl;
    let maybe_user: Result<User<'_>, _> = user_dsl::users
        .select(User::as_select())
        .filter(
            user_dsl::password.eq(login.password)
                .and(user_dsl::email.eq(login.email))
        )
        .get_result(&mut conn)
        .await;

    use crate::schema::users_metadata::dsl as meta_dsl;

    match maybe_user {
        Err(err) => {
            panic!("Error logging in: {err}");
        },
        Ok(user) => {
            let user_meta = meta_dsl::users_metadata
                .select(UserMetadata::as_select())
                .filter(meta_dsl::id.eq(user.id))
                .get_result(&mut conn)
                .await
                .unwrap();

            return user.into();
        }
    };
}