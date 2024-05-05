use rocket::Route;

pub mod login;
pub use login::*;

pub mod logout;
pub use logout::*;

pub mod register;
pub use register::*;

pub fn routes() -> Vec<Route> {
    let routes = routes![login::login, logout::logout, register::register];
    routes
}
