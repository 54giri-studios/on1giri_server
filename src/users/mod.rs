//! Functions / Routes used interact with user data
mod routes;
pub use routes::*;

mod types;
pub use types::*;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        get_by_id,
        get_by_username_discriminator,

        delete_by_id,
        delete_by_username_discriminator,

        update_by_id,
        update_by_username_discriminator,
    ]
}
