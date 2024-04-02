use crate::roles::Role;
use crate::channels::Channel;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    id: i32,
    name: String,
    owner_id: i32,
    roles: Vec<Role>,
    channels: Vec<Channel>,
    description: String,
    creation_date: DateTime<Utc>
}
