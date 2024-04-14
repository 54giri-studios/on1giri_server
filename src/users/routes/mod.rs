mod get;
pub use get::*;

mod delete;
pub use delete::*;

mod patch;
pub use patch::*;

mod post;
pub use post::*;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut routes = routes![];
    routes.extend(routes![delete_by_id]);
    routes.extend(routes![get_by_id]);
    routes.extend(routes![user_create]);

    routes

}
