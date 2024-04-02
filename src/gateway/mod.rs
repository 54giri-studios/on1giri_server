use rocket::Route;

mod get;
pub use get::*;

mod types;
pub use types::*;

pub fn routes() -> Vec<Route> {
    routes![]
}
