use diesel::prelude::*;

use rocket::{
    http::Status, response::stream::{Event, EventStream}, serde::json::Json, tokio::{
        select,
        sync::broadcast::{self, error::RecvError, Sender},
    }, Error, Shutdown, State
};

use crate::{AppState, Channel, ChannelMessage, ChannelPermissions, DbPool, ErrorResponse, JsonResponse, NewChannel, Role, RoleCategory};
use crate::schema::{
    channels::dsl as c,
    channel_permissions::dsl as cp,
    members_roles::dsl as mr,
    roles::dsl as r,
};

use diesel_async::{scoped_futures::ScopedFutureExt, RunQueryDsl};


/// Retrieves information about a single channel
/// Refer to [crate::Channel] for the return type
#[get("/<channel_id>")]
pub async fn get_channel(pool: &State<DbPool>, channel_id: i32) -> Result<Json<Channel>, Json<ErrorResponse>> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            return Err(ErrorResponse::internal_error(err).into())
        }
    };

    use crate::schema::channels::dsl::*;
    let query: Result<Channel, diesel::result::Error> = channels
        .select(Channel::as_select())
        .filter(id.eq(channel_id))
        .get_result(&mut conn)
        .await;

    query
        .map_err(ErrorResponse::from)
        .map(|e| e.into())
        .map_err(|a| a.into())
}

#[post("/create", format = "json", data = "<new_channel>")]
pub async fn post_channel<'a>(
    pool: &State<DbPool>, 
    new_channel: Json<NewChannel<'a>>
) -> JsonResponse<Channel> {
    let mut conn = match pool.get().await {
        Ok(pool) => pool,
        Err(err) => return Err(ErrorResponse::internal_error(err).into()),
    };

    let maybe_channel: Result<Channel, ErrorResponse> = conn
        .build_transaction()
        .serializable()
        .run(move |conn| async {
            let channel: Channel = diesel::insert_into(c::channels)
                .values(new_channel.into_inner())
                .returning(Channel::as_returning())
                .get_result(conn)
                .await
                .map_err(|err| ErrorResponse::new(
                        Status::InternalServerError, 
                        format!("Couldn't create this channel: {err}")
                ))?;

            let guild_roles: Vec<Role> = r::roles
                .filter(r::guild_id.eq(channel.guild_id()))
                .get_results(conn)
                .await
                .map_err(
                    |err| ErrorResponse::new(
                        Status::InternalServerError,
                        format!("Failed to query this guild's roles: {err}"),
                ))?;

            let permissions: Vec<ChannelPermissions> = guild_roles
                .into_iter()
                .map(|r| ChannelPermissions::all_allowed(r.id(), r.guild_id(), channel.id()))
                .collect();

            diesel::insert_into(cp::channel_permissions)
                .values(permissions)
                .execute(conn)
                .await
                .map_err(
                    |err| ErrorResponse::new(
                        Status::InternalServerError,
                        format!("Failed to setup permissions for this channel: {err}")
                ))?;

            Ok(channel)

        }.scope_boxed())
        .await;

    match maybe_channel {
        Ok(channel) => Ok(channel.into()),
        Err(err) => Err(err.into())
    }
}

#[get("/<channel_id>/subscribe")]
pub async fn subscribe(
    channel_id: i32,
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
