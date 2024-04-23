use diesel::prelude::*;

#[derive(Debug, Queryable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::members_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MemberRole {
    role_id: i32,
    guild_id: i32,
    member_id: i32
}

impl MemberRole {
    pub fn new(role_id: i32, guild_id: i32, member_id: i32) -> Self {
        Self { role_id, guild_id, member_id }
    }
}