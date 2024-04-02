use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Message {
    id: i32,
    channel_id: i32,
    author_id: i32,
    content: String,
    creation_date: DateTime<Utc>
}
