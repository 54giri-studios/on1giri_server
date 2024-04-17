//! Functions / Routes used interact with user data
use chrono::DateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use std::env;

mod routes;
pub use routes::*;

pub mod types;
pub use types::*;


use crate::DbPool;

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

    let email = env::var("OVERLORD_EMAIL").expect("OVERLORD_EMAIL env var must be set");
    let password = env::var("OVERLORD_PASSWORD").expect("OVERLORD_PASSWORD env var must be set");
    let ov_access_level = env::var("OVERLORD_ACCESS_LEVEL").expect("OVERLORD_ACCESS_LEVEL env var must be set");

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

    use crate::schema::users_metadata::dsl as m_dsl;

    let username = env::var("OVERLORD_USERNAME").expect("OVERLORD_USERNAME must be set");
    let discriminator: i16 = env::var("OVERLORD_DISCRIMINATOR")
        .expect("OVERLORD_DISCRIMINATOR env var must be set")
        .parse()
        .unwrap();
    let last_check_in = {
        let lci_str = env::var("OVERLORD_LAST_CHECK_IN")
            .expect("OVERLORD_LAST_CHECK_IN env var must be set");
        DateTime::from_timestamp(
            lci_str.parse().unwrap(), 
            0
        ).unwrap()
    };
    let picture = env::var("OVERLORD_PICTURE").expect("OVERLORD_PICTURE env var must be set");

    let account_creation = {
        let lci_str = env::var("OVERLORD_ACCOUNT_CREATION").expect("OVERLORD_ACCOUNT_CREATION env var must be set");
        DateTime::from_timestamp(
            lci_str.parse().unwrap(), 
            0
        ).unwrap()
    };
    let description = env::var("OVERLORD_DESCRIPTION").expect("OVERLORD_DESCRIPTION env var must be set");
    let overlord_meta = UserMetadata {
        id: 0,
        username,
        discriminator,
        last_check_in,
        picture,
        account_creation,
        description
    };

    diesel::insert_into(m_dsl::users_metadata)
        .values(&overlord_meta)
        .on_conflict(m_dsl::id)
        .do_update()
        .set(&overlord_meta)
        .execute(&mut connection)
        .await
        .unwrap();

    Ok(())
}
