use rocket::{route, Route};

pub mod types;
pub use types::*;

mod routes;
pub use routes::routes;
