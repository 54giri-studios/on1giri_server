#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod users;
mod gateway;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/users/", users::routes())
        .mount("/gateway/", gateway::routes())
}


