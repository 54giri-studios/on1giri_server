//! Functions / routes used to retrieve informations about users

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{DbPool, ErrorResponse, Guild, User};

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
            Json::from(user)
        }
        Err(e) => {
            todo!();
        }
    }
}

#[get("/<user_id>/guilds")]
pub async fn get_guilds(pool: &State<DbPool>, user_id: i32) -> Result<Json<Vec<Guild>>, Json<ErrorResponse>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(ErrorResponse::internal_error(err).into())
        }
    };

    use crate::schema::members::dsl as m_dsl;
    use crate::schema::guilds::dsl as g_dsl;

    let maybe_guilds: Result<Vec<Guild>, _> = m_dsl::members
        .inner_join(g_dsl::guilds)
        .select(Guild::as_select())
        .filter(m_dsl::user_id.eq(user_id))
        .get_results(&mut conn)
        .await;

    match maybe_guilds {
        Ok(guilds) => Ok(guilds.into()),
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}
