use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use rocket::{serde::json::Json, State};

use crate::{DbPool, ErrorResponse, Role};

use crate::schema::roles::dsl as r;

#[get("/<guild_id>/roles")]
pub async fn get_guild_roles(
    pool: &State<DbPool>,
    guild_id: i32,
) -> Result<Json<Vec<Role>>, Json<ErrorResponse>> {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(err) => return Err(ErrorResponse::internal_error(err).into())       
    };

    let maybe_roles: Result<Vec<Role>, _> = r::roles
        .filter(r::guild_id.eq(guild_id))
        .get_results(&mut conn)
        .await;

    let roles: Vec<Role> = match maybe_roles {
        Ok(r) => r,
        Err(err) => return Err(ErrorResponse::from(err).into()),
    };

    Ok(roles.into())
}
