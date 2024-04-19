use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{
    response::stream::{Event, EventStream},
    serde::json::Json,
    tokio::{
        select,
        sync::broadcast::{self, error::RecvError, Sender},
    },
    Shutdown, State,
};

use crate::{AppState, Channel, DbPool, ChannelMessage};

#[get("/<channel_id>")]
pub async fn get_channel(pool: &State<DbPool>, channel_id: i32) -> Json<Channel> {
    let mut conn = pool.get().await.unwrap();

    use crate::schema::channels::dsl::*;
    let channel = channels
        .select(Channel::as_select())
        .filter(id.eq(channel_id))
        .get_result(&mut conn)
        .await
        .unwrap();

    channel.into()
}

#[get("/<channel_id>/subscribe")]
pub async fn subscribe(
    channel_id: u32,
    sessions: &State<AppState>,
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

    let queue: &Sender<ChannelMessage> = &new_cli;

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
