use std::{collections::HashMap, sync};

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
