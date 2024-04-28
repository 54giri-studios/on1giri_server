use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, RunQueryDsl};
use rocket::{serde::json::Json, State};

use crate::{DbPool, ErrorResponse, Guild, InsertableGuild, Member, MemberRole, NewRole, Newguild, Role};

use crate::schema::{
    guilds::dsl as g_dsl,
    members::dsl as m_dsl,
    members_roles::dsl as mr_dsl,
    roles::dsl as r_dsl,
};

#[post("/create", format = "json", data = "<new_guild>")]
pub async fn post_guild(
    pool: &State<DbPool>, 
    new_guild: Json<Newguild>,
) -> Result<Json<Guild>, Json<ErrorResponse>> {

    let mut conn = match pool.get().await {
        Ok(pool) => pool,
        Err(e) => return Err(ErrorResponse::internal_error(e).into()),
    };

    let insertable_guild = InsertableGuild::new(new_guild.into_inner());

    let maybe_guild: Result<Guild, diesel::result::Error> = conn
        .build_transaction()
        .serializable()
        .run(|conn| async move {
            let guild: Guild = diesel::insert_into(g_dsl::guilds)
                .values(insertable_guild)
                .returning(Guild::as_returning())
                .get_result(conn)
                .await?;

            let owner = Member::new(guild.owner_id(), guild.id());
            let owner: Member = diesel::insert_into(m_dsl::members)
                .values(owner)
                .returning(Member::as_returning())
                .get_result(conn)
                .await?;

            let default_role: Role = diesel::insert_into(r_dsl::roles)
                .values(NewRole::everyone(guild.id()))
                .returning(Role::as_returning())
                .get_result(conn)
                .await?;

            diesel::insert_into(mr_dsl::members_roles)
                .values(MemberRole::new(default_role.id(), guild.id(), owner.user_id()))
                .execute(conn)
                .await?;

            let owner_role: Role = diesel::insert_into(r_dsl::roles)
                .values(NewRole::owner(guild.id()))
                .returning(Role::as_returning())
                .get_result(conn)
                .await?;

            diesel::insert_into(mr_dsl::members_roles)
                .values(MemberRole::new(owner_role.id(), guild.id(), owner.user_id()))
                .execute(conn)
                .await?;

            Ok(guild)
        }.scope_boxed()
    ).await;

    match maybe_guild {
        Ok(g) => Ok(g.into()),
        Err(err) => Err(ErrorResponse::from(err).into())
    }
}
