use chrono::{DateTime, Utc};
use rocket::{data::{self, FromData}, Request, Data};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: i32,
    channel_id: i32,
    author_id: i32,
    content: String,
    creation_date: DateTime<Utc>
}