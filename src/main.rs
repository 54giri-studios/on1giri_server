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
use gateway::SubscriptionState;

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

type Db = AsyncDieselConnectionManager<AsyncPgConnection>;
type DbPool = Pool<AsyncPgConnection>;

mod gateway;

fn establish_db_connection() -> DbPool {
    dotenvy::dotenv().ok();
    
    let config = Db::new(
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
    let mut subscriptions = SubscriptionState::new();

    rocket::build()
        .manage(pool)
        .manage(subscriptions)
        .mount("/users/", users::routes())
        .mount("/gateway/", gateway::routes())
}


