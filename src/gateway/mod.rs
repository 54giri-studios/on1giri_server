use rocket::Route;

mod get;
use get::echo_channel;

pub fn routes() -> Vec<Route> {
    routes![echo_channel]
}
