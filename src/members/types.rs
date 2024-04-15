pub struct Member {
    pub user_id: i32,
    pub guild_id: i32
}

impl Member {
    pub fn new(user_id: i32, guild_id: i32) -> Self {
        Self { user_id, guild_id }
    }
}
