use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable, Queryable, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    id: i32,
    guild_id: i32,
    name: String
}

impl Role {
    pub fn new(id: i32, guild_id: i32, name: String) -> Self {
        Self { id, guild_id, name }
    }
}