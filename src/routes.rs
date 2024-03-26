use crate::server;
use rocket::{
    response::stream::{Event, EventStream},
    serde::json::Json,
    tokio::{
        select,
        sync::broadcast::{self, error::RecvError, Sender},
    },
    Shutdown, State,
};

#[get("/subscribe/<channel_id>")]
pub async fn subscribe(
    channel_id: u32,
    sessions: &State<server::AppState>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut sessions = sessions.clients.lock().await;
    // Does the sessions exists ?
    // If not we create it and store in the state (meaning new conversation)
    let new_cli = if let Some(existing_cli) = sessions.get(&channel_id) {
        existing_cli.clone()
    } else {
        let new_cli = broadcast::channel(10).0;
        sessions.insert(channel_id, new_cli.clone());
        new_cli
    };

    let queue: &Sender<server::Message> = &new_cli;

    // Add the client to the broadcast channel so that he can receive messages
    let mut client = queue.subscribe();

    EventStream! {
        // Messages received down the channel are sent to the clients that
        // they've subscribed to
        loop {
            let msg = select! {
                msg  = client.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[post("/publish", format = "json", data = "<message>")]
pub async fn publish(sessions: &State<server::AppState>, message: Json<server::Message>) {
    let message = message.into_inner();
    let room_id = message.room_id;
    let mut sessions = sessions.clients.lock().await;
    // If the channel exists, send the message to the channel
    if let Some(queue) = sessions.get(&room_id) {
        // save to the database and send the message to the channel
        // if the channel doesn't have any subscribers, check the type of the message
        // if the message is a SEND save it to the database...
        if message.message_type == server::MessageType::QUIT && queue.receiver_count() <= 1 {
            sessions.remove(&room_id);
        } else if let Err(_) = queue.send(message) {
            sessions.remove(&room_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn test_subscribe() {
        // Create a Rocket instance and a Client
        let rocket = rocket();
        let client = Client::tracked(rocket).unwrap();

        // Generate a random channel_id
        let channel_id = 2;

        // Create and dispatch a GET request to the subscribe route
        let response = client.get(uri!(subscribe(channel_id))).dispatch();

        // Check that the response status is OK
        assert_eq!(response.status(), Status::Ok);

        // Check that the response content type is EventStream
        assert_eq!(
            response.content_type(),
            Some(ContentType::new("text", "event-stream"))
        );
    }

    #[test]
    fn test_publish() {
        // Create a Rocket instance and a Client
        let rocket = rocket();
        let client = Client::tracked(rocket).unwrap();

        let _ = client.get(uri!(subscribe(2))).dispatch();

        // Create and dispatch a POST request to the publish route
        let response = client
            .post(uri!(publish))
            .header(ContentType::JSON)
            .body(r#"{ "room_id": 2, "message_type": "SEND", "message_content": "Hello, world!" }"#)
            .dispatch();

        // Check that the response status is OK
        assert_eq!(response.status(), Status::Ok);

        // Check that the response content type is empty
        assert_eq!(response.content_type(), None);
    }
}
