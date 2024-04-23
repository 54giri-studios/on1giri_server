mod post;
pub use post::*;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut routes = routes![];
    routes.extend(routes![post_message]);

    routes
}
