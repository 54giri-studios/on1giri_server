#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;

use chrono::TimeDelta;
use diesel_async::{
    pooled_connection::{
        deadpool::{BuildError, Pool}, AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};

mod channels;
pub use channels::types::*;

mod guilds;
pub use guilds::types::*;

mod login;
pub use login::types::*;

mod members;

mod messages;
pub use messages::types::*;


mod roles;
pub use roles::types::*;

mod users;
pub use users::types::*;


type Db = AsyncDieselConnectionManager<AsyncPgConnection>;
type DbPool = Pool<AsyncPgConnection>;

mod gateway;

pub mod schema; 

fn establish_db_connection() -> Result<DbPool, BuildError> {
    dotenvy::dotenv().ok();
    
    let config = Db::new(
        std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("DATABASE_URL env var must be set"))
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

    if let Err(err) = channels::setup(&pool).await {
        panic!("Error setting up the system channel {err}");
    };


    let token_handler = TokenHandler::new(TimeDelta::days(7))
        .unwrap_or_else(|| panic!("Failed to generate the token handler"));

    rocket::build()
        .manage(pool)
        .manage(token_handler)
        .mount("/channels/", channels::routes())
        .mount("/gateway/", gateway::routes())
        .mount("/guilds/", guilds::routes())
        .mount("/login/", login::routes())
        .mount("/messages/", messages::routes())
        .mount("/users/", users::routes())
}
