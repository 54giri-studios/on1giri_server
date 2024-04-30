use rocket::Route;

pub mod channels;
pub use channels::*;

pub mod get;
pub use get::*;

pub mod guilds;
pub use guilds::*;

pub mod post;
pub use post::*;

pub mod members;
pub use members::*;

pub mod permissions;
pub use permissions::*;

pub mod roles;
pub use roles::*;

pub fn routes() -> Vec<Route> {
    let routes = routes![
        guilds::get_guild,
        guilds::patch_guild,
        roles::get_guild_roles,
        channels::get_guild_channels,
        members::get_guild_member,
        members::get_guild_members,

        permissions::get_guild_channel_permissions,
        permissions::get_guild_channel_role_permissions,
        permissions::patch_guild_channel_role_permissions,

        post::post_guild,
    ];
    routes
}
