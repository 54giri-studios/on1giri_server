#[macro_use] extern crate rocket;
mod users;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/users/", users::routes())
}


