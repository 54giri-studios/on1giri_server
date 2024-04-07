//! Functions / Routes used interact with user data
mod routes;
use diesel::{query_dsl::methods::FilterDsl, Connection, SelectableHelper};
use diesel_async::RunQueryDsl;
pub use routes::*;

mod types;
pub use types::*;

use rocket::{http::hyper::server::conn::AddrIncoming, Route};

use crate::{schema::users::access_level, users, DbPool};

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

    let overlord = User {
        id: 0,
        password: "admin".into(),
        access_level: "admin".into(), 
        email: "admin@admin.com".into()
    };

    use crate::schema::users::{self, id};
    let returned = diesel::insert_into(users::table)
        .values(&overlord)
        .on_conflict(id)
        .do_nothing()
        .execute(&mut connection)
        .await?;

    Ok(())
}
