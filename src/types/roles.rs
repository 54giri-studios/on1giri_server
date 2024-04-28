use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{ChannelPermissions, Color, RoleCategory};

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRole {
    guild_id: i32,
    name: String,
    color: String,
    category: String
}

impl NewRole {

    pub fn new(guild_id: i32, name: String, color: String, category: String) -> Self {
        Self { guild_id, name, color, category }
    }

    pub fn everyone(guild_id: i32) -> Self {
        let default_role_category = RoleCategory::everyone().to_string();
        let default_role_color = Color::default_role_color().to_hex_string();

        Self::new(guild_id, default_role_category.clone(), default_role_color, default_role_category)
    }

    pub fn owner(guild_id: i32) -> Self {
        let owner_role_category = RoleCategory::owner().to_string();
        let owner_role_color = Color::owner_role_color().to_hex_string();

        Self::new(guild_id, owner_role_category.clone(), owner_role_category.clone(), owner_role_category)

    }
}


#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable, Queryable, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    id: i32,
    guild_id: i32,
    name: String,
    color: String,
    category: String,
}

impl Role {
    pub fn new(id: i32, guild_id: i32, name: String, color: String, category: String) -> Self {
        Self { id, guild_id, name, color, category }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}