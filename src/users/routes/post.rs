use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{schema::users, users::NewUser, DbPool, User};

#[post("/", format = "json", data = "<new_user>")]
pub async fn post_user<'a>(pool: &State<DbPool>, new_user: Json<NewUser<'a>>) -> Json<User<'a>> {
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