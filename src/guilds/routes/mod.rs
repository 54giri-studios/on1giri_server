use rocket::Route;

mod get;
mod post;

pub fn routes() -> Vec<Route> {
    let routes = routes![
        get::get_channels,
        get::get_permissions,
        get::get_permissions_for_role,
        post::post_guild,
    ];
    routes
}
