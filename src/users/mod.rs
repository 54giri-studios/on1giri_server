//! Functions / Routes used interact with user data
mod routes;
use diesel::{query_dsl::methods::FilterDsl, Connection, SelectableHelper};
use diesel_async::RunQueryDsl;
pub use routes::*;

mod types;
pub use types::*;

use rocket::{http::hyper::server::conn::AddrIncoming, Route};

use crate::{schema::users::access_level, users, DbPool};

pub async fn setup(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = pool.get().await?;


    use crate::schema::access_levels;

    let levels = [AccessLevel::admin(), AccessLevel::regular()];

    for lv in levels {
        diesel::insert_into(access_levels::table)
            .values(&lv)
            .on_conflict(access_levels::level)
            .do_nothing()
            .execute(&mut connection)
            .await?;
    }

    let overlord = User::new(
            0,
        "admin@admin.com".into(),
    "admin".into(),
            "admin".into()
        );

    use crate::schema::users::{self, id};
    diesel::insert_into(users::table)
        .values(&overlord)
        .on_conflict(id)
        .do_nothing()
        .execute(&mut connection)
        .await?;

    Ok(())
}
