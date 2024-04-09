#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelKind {
    Category = 0,
    Text
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: i32,
    guild_id: i32,
    name: String,
    kind: ChannelKind
}