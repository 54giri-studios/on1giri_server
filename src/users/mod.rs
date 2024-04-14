//! Functions / Routes used interact with user data
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use std::env;

mod routes;
pub use routes::*;

mod types;
pub use types::*;

use rocket::{http::hyper::server::conn::AddrIncoming, Route};

use crate::{schema::users::access_level, users, DbPool};

pub async fn setup(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = pool.get().await?;


    use crate::schema::access_levels::dsl as al_dsl;

    let levels = [AccessLevel::admin(), AccessLevel::regular()];

    for lv in levels {
        diesel::insert_into(al_dsl::access_levels)
            .values(&lv)
            .on_conflict(al_dsl::level)
            .do_nothing()
            .execute(&mut connection)
            .await?;
    }

    let email = env::var("OVERLORD_EMAIL").unwrap();
    let password = env::var("OVERLORD_PASSWORD").unwrap();
    let ov_access_level = env::var("OVERLORD_ACCESS_LEVEL").unwrap();

    let overlord = User {
        id: 0,
        password: password.into(),
        access_level: ov_access_level.into(),
        email: email.into()
    };

    use crate::schema::users::dsl as u_dsl;
    diesel::insert_into(u_dsl::users)
        .values(&overlord)
        .on_conflict(u_dsl::id)
        .do_update()
        .set((
            u_dsl::email.eq(&overlord.email),
            u_dsl::password.eq(&overlord.password),
            u_dsl::access_level.eq(&overlord.access_level),
        ))
        .execute(&mut connection)
        .await?;

    Ok(())
}
