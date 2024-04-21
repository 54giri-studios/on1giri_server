use rocket::Route;

mod get;
mod post;

pub fn routes() -> Vec<Route> {
    let routes = routes![
        get::get_channels,
        post::post_guild,
    ];
    routes
}
