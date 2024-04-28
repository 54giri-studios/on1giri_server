use rocket::Route;

pub mod login;
pub mod logout;
pub mod register;

pub fn routes() -> Vec<Route> {
    let routes = routes![login::login, logout::logout, register::register];
    routes
}
