//! Functions / Routes used interact with user data
use chrono::DateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use std::env;

mod routes;
pub use routes::*;

use crate::{AccessLevel, DbPool, User, UserMetadata};

pub async fn setup(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = pool.get().await?;

    use crate::schema::access_levels::dsl as al_dsl;

    let levels = [AccessLevel::admin(), AccessLevel::regular()];

    diesel::insert_into(al_dsl::access_levels)
        .values(&levels)
        .on_conflict(al_dsl::level)
        .do_nothing()
        .execute(&mut connection)
        .await?;

    Ok(())
}
