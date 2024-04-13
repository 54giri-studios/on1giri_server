use rocket::Route;

mod get;

pub fn routes() -> Vec<Route> {
    let mut routes = routes![];

    routes.extend(routes![get::get_channel]);

    routes
}
