use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{DbPool, ErrorResponse, Guild, InsertableGuild, Newguild};

#[post("/create", format = "json", data = "<new_guild>")]
pub async fn post_guild<'a>(
    pool: &State<DbPool>, 
    new_guild: Json<Newguild<'a>>,
) -> Result<Json<Guild<'a>>, Json<ErrorResponse>> {

    let mut conn = match pool.get().await {
        Ok(pool) => pool,
        Err(e) => return Err(ErrorResponse::internal_error(e).into()),
    };

    let insertable_guild = InsertableGuild::new(new_guild.into_inner());

    use crate::schema::guilds::dsl as g_dsl;
    let maybe_guild: Result<Guild<'a>, _> = diesel::insert_into(g_dsl::guilds)
        .values(insertable_guild)
        .returning(Guild::as_returning())
        .get_result(&mut conn)
        .await;

    match maybe_guild {
        Ok(guild) => Ok(guild.into()),
        Err(err) => Err(ErrorResponse::from(err).into()),
    }
}