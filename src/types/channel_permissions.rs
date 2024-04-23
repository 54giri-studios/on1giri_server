use diesel::prelude::*;

use crate::{Guild, Role, Channel};

#[derive(Debug, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = crate::schema::channel_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewChannelPermissions {
    can_read: Option<bool>,
    can_write: Option<bool>
}

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

    pub fn nothing_allowed(role_id: i32, guild_id: i32, channel_id: i32) -> Self {
        Self { role_id, guild_id, channel_id, can_read: false, can_write: false }
    }
}
#[derive(Debug, Serialize)]
pub struct PopulatedChannelPermissions {
    role: Role,
    guild: Guild,
    channel: Channel,
    can_read: bool,
    can_write: bool,
}

impl PopulatedChannelPermissions {
    pub fn new(
        permissions: ChannelPermissions,
        role: Role,
        guild: Guild,
        channel: Channel
    ) -> Self {
        Self { 
            role, 
            guild, 
            channel,
            can_read: permissions.can_read,
            can_write: permissions.can_write
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ChannelPermissionsForRole {
    role: Role,
    guild_id: i32,
    channel_id: i32,
    can_read: bool,
    can_write: bool,
} 

impl ChannelPermissionsForRole {
    pub fn new(perms: ChannelPermissions, role: Role) -> Self {
        Self {
            role,
            guild_id: perms.guild_id,
            channel_id: perms.channel_id,
            can_read: perms.can_read,
            can_write: perms.can_write
        }
    }
}
