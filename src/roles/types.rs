use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    id: i32,
    guild_id: i32,
    name: String
}
