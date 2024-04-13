use rocket::Route;

mod get;

pub fn routes() -> Vec<Route> {
    let mut routes = routes![get::get_channels];
    routes
}