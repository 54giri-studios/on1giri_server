use rocket::{response::stream::EventStream, Shutdown, State};
use super::SubscriptionState;

pub async fn subscribe(
    channel_id: u32,
    subscriptions: &State<SubscriptionState>,
    mut end: Shutdown,
) {
    todo!();
}
