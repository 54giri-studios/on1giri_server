use diesel::prelude::*;

use crate::{Guild, Role, User, UserMetadata};

#[derive(Debug, AsChangeset, Selectable, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = crate::schema::members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Member {
    pub user_id: i32,
    pub guild_id: i32,
    // Probably some extra data lol ...
}

impl Member {
    pub fn new(user_id: i32, guild_id: i32) -> Self {
        Self { user_id, guild_id }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[derive(Debug, Serialize)]
pub struct PopulatedMember {
    #[serde(flatten)]
    user: UserMetadata,
    guild: Guild,
    roles: Vec<Role>,
}

impl PopulatedMember {
    pub fn new(user: UserMetadata, guild: Guild, roles: Vec<Role>) -> Self {
        Self { user, guild, roles }
    }
}
