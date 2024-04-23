use chrono::Utc;
use diesel_async::RunQueryDsl;

mod routes;
pub use routes::routes;

use crate::{DbPool, Guild};

pub async fn setup(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}