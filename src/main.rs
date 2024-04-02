#![allow(unused)]

#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;

use diesel_async::{
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::Pool
    },
    AsyncConnection, 
    AsyncPgConnection,
    RunQueryDsl
};

use std::env;

mod channels;
pub use channels::Channel;

mod guilds;
pub use guilds::Guild;

mod messages;
pub use messages::Message;

mod users;
pub use users::{User, UserMetadata};

mod roles;
pub use roles::Role;

type DbPool = AsyncDieselConnectionManager<AsyncPgConnection>;

mod gateway;

fn establish_db_connection() -> Pool<AsyncPgConnection> {
    dotenvy::dotenv().ok();
    
    let config = DbPool::new(
        std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("DATABASE_URL env var must be set"))
    );

    Pool::builder(config)
        .build()
        .unwrap()
}


#[launch]
async fn rocket() -> _ {
    let mut pool = establish_db_connection();

    rocket::build()
        .manage(pool)
        .mount("/users/", users::routes())
        .mount("/gateway/", gateway::routes())
}


