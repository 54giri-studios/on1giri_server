use rocket::Route;

mod channels;
mod messages;
mod members;


pub fn routes() -> Vec<Route> {
    let mut routes = routes![];

    routes.extend(routes![messages::get_channel_history, messages::get_message]);
    routes.extend(routes![members::get_channel_members]);
    routes.extend(routes![channels::subscribe]);
    routes.extend(routes![channels::get_channel, channels::post_channel]);

    routes
}
