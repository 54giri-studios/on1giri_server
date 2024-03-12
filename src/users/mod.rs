//! Functions / Routes used interact with user data
use rocket::Route;

mod get;
pub use get::{get_by_id, get_by_username_discriminator};

mod delete;
pub use delete::{delete_by_id, delete_by_username_discriminator};

mod patch;

pub enum Permission {
    Admin,
    Regular,
}

pub struct User {
    id: i64,
    password_hash: String,
    user_type: Permission,
    email: String,
}



/// Return all routes associated to the current folder
pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();

    let get_routes = routes![get_by_id, get_by_username_discriminator];
    routes.extend(get_routes);

    let delete_routes = routes![delete_by_id, delete_by_username_discriminator];
    routes.extend(delete_routes);

    routes
}
