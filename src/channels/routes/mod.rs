use rocket::Route;

mod messages;
mod channels;

mod permissions;

pub fn routes() -> Vec<Route> {
    let mut routes = routes![];

    routes.extend(routes![messages::get_channel_history, messages::get_message]);
    routes.extend(routes![channels::subscribe]);
    routes.extend(routes![channels::get_channel, channels::post_channel]);

    routes
}
