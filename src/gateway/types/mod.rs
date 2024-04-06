use std::{collections::HashMap, hash::Hash, sync::{self, Mutex}};

use diesel::dsl::HasAnyKeyJsonb;
use rocket::futures::channel::mpsc::Sender;

pub enum MessageType {
    Connect,
    Send,
    Quit
}

pub struct SubscriptionMessage {

}

pub struct SubscriptionState {
    pub clients: sync::Mutex<HashMap<u32, Sender<SubscriptionMessage>>>
}

impl SubscriptionState {
    pub fn new() -> Self {
        Self {
            clients: Mutex::new(HashMap::new())
        }
    }
}
