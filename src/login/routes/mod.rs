use rocket::Route;

mod post;

pub fn routes() -> Vec<Route> {
    let mut routes = routes![];

    routes.extend(routes![post::login]);

    routes
}
