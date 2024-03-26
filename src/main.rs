#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket::{tokio::sync::Mutex, Build, Rocket};

pub mod routes;
pub mod server;

#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(server::AppState{
            clients: Mutex::new(HashMap::new()),
        })
        .mount("/", routes![routes::subscribe, routes::publish])
}
