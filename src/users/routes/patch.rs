//! Functions / routes used to update informations about users

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use rocket::{serde::json::Json, State};

use crate::{DbPool, ErrorResponse, JsonResponse, PatchUserMetadata, UserMetadata};
use crate::schema::users_metadata::dsl as um;

/// Updates an user from its id
/// 
/// # Arguments
/// * `id` - The user's unique identifier
#[patch("/<user_id>", data = "<new_meta>")]
pub async fn patch_user<'a>(pool: &State<DbPool>, user_id: i32, new_meta: Json<PatchUserMetadata<'a>>) -> JsonResponse<UserMetadata> {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(err) => return Err(ErrorResponse::internal_error(err).into())
    };

    let maybe_um: Result<UserMetadata, _> = diesel::update(um::users_metadata)
        .filter(um::id.eq(user_id))
        .set(new_meta.into_inner())
        .returning(UserMetadata::as_returning())
        .get_result(&mut conn)
        .await;

    match maybe_um {
        Ok(um) => Ok(um.into()),
        Err(err) => Err(ErrorResponse::from(err).into()),
    }
}

