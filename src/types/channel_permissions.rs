use diesel::prelude::*;

use crate::{Guild, Role, Channel};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable, Selectable, Queryable, QueryableByName)]
#[diesel(table_name = crate::schema::channel_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelPermissions {
    role_id: i32,
    guild_id: i32,
    channel_id: i32,
    can_read: bool,
    can_write: bool,
}

impl ChannelPermissions {
    pub fn all_allowed(role_id: i32, guild_id: i32, channel_id: i32) -> Self {
        Self { role_id, guild_id, channel_id, can_read: true, can_write: true }
    }
}
#[derive(Debug, Serialize)]
pub struct PopulatedChannelPermissions {
    #[serde(flatten)]
    permissions: ChannelPermissions,
    role: Role,
    guild: Guild,
    channel: Channel,
}

impl PopulatedChannelPermissions {
    pub fn new(
        permissions: ChannelPermissions,
        role: Role,
        guild: Guild,
        channel: Channel
    ) -> Self {
        Self { permissions, role, guild, channel }
    }
}
