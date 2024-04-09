mod get;
pub use get::*;

mod post;
pub use post::*;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut routes = routes![];
    routes.extend(routes![get_message]);
    routes.extend(routes![post_message]);

    routes
}
