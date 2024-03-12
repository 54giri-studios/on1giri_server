#[macro_use] extern crate rocket;
mod users;

mod gateway;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", gateway::routes())
        .mount("/users/", users::routes())
}


