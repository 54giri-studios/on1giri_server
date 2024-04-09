mod types;
use rocket::{route, Route};
pub use types::*;

mod routes;
pub use routes::routes;
