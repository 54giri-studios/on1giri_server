use rocket::Route;

mod get;
mod post;

pub fn routes() -> Vec<Route> {
    let mut routes = routes![];

    routes.extend(routes![get::get_channel]);
    routes.extend(routes![post::get_channel_history]);

    routes
}
