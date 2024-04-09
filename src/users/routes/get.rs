//! Functions / routes used to retrieve informations about users

use diesel::{query_dsl::methods::FilterDsl, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{DbPool, User};

/// Gets an user from its id
/// 
/// # Arguments
/// * `id` - The user's unique identifier
#[get("/<id>")]
pub async fn get_by_id<'a>(pool: &State<DbPool>, id: i32) -> Json<User<'a>> {
    use crate::schema::users::dsl::users;

    let mut conn = pool.get().await.unwrap();

    let maybe_user: Result<User<'a>, _> = users
        .select(User::as_select())
        .find(id)
        .first(&mut conn)
        .await;

    match maybe_user {
        Ok(user) => {
            return Json::from(user);
        }
        Err(e) => {
            todo!();
        }
    }
}

/// Gets an user using it's name and discriminator
/// 
/// # Arguments
/// * `username` - The user's current username
/// * `discriminator` - The user's current discriminator, must be between 0 and 9999 included
#[get("/<username>/<discriminator>")]
pub async fn get_by_username_discriminator(username: &str, discriminator: u8) {
}

