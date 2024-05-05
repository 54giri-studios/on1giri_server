#![allow(unused)]
#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;


use chrono::TimeDelta;
use diesel_async::pooled_connection::deadpool::{BuildError, Pool};
use std::collections::HashMap;

use rocket::{local::asynchronous::Client, tokio::sync::Mutex};

mod channels;
mod guilds;
mod auth;
mod members;
mod messages;
mod roles;
mod setup;
mod users;

mod types;
pub use types::*;

mod gateway;

pub mod schema;

fn establish_db_connection() -> Result<DbPool, BuildError> {
    dotenvy::dotenv().ok();

    let config = Db::new(
        std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("DATABASE_URL env var must be set")),
    );

    Pool::builder(config).build()
}

#[launch]
async fn rocket() -> _ {
    let pool = match establish_db_connection() {
        Ok(p) => p,
        Err(err) => {
            panic!("Failed to connect to the database: {err}");
        }
    };

    if let Err(err) = users::setup(&pool).await {
        panic!("Error setting up the overlord {err}");
    };

    if let Err(err) = guilds::setup(&pool).await {
        panic!("Error setting up the system guild {err}");
    };

    if let Err(err) = members::setup(&pool).await {
        panic!("Error setting up the default member: {err}");
    }

    if let Err(err) = channels::setup(&pool).await {
        panic!("Error setting up the system channel {err}");
    };

    if let Err(err) = roles::setup(&pool).await {
        panic!("Error setting up roles {err}");
    }

    let token_handler = TokenHandler::new(TimeDelta::days(7))
        .unwrap_or_else(|| panic!("Failed to generate the token handler"));

    /*
    if let Err(err) = setup::setup_system(&pool, &token_handler).await {
        panic!("Error setting up system {err}");
    }
    */

    rocket::build()
        .manage(pool)
        .manage(token_handler)
        .manage(AppState {
            clients: Mutex::new(HashMap::new()),
        })
        .mount("/channels/", channels::routes())
        .mount("/gateway/", gateway::routes())
        .mount("/guilds/", guilds::routes())
        .mount("/auth/", auth::routes())
        .mount("/messages/", messages::routes())
        .mount("/users/", users::routes())

}
