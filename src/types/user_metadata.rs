use diesel::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Queryable, Insertable, Selectable, Serialize, AsChangeset, PartialEq)]
#[diesel(table_name = crate::schema::users_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserMetadata {
    pub id: i32,
    pub username: String,
    pub discriminator: i16,
    pub last_check_in: DateTime<Utc>,
    pub picture: String,
    pub account_creation: DateTime<Utc>,
    pub description: String
}

impl UserMetadata {
    pub fn user_id(&self) -> &i32 {
        &self.id
    }

    pub fn new(
        id: i32,
        username: String,
        discriminator: i16,
        last_check_in: DateTime<Utc>,
        picture: String,
        account_creation: DateTime<Utc>,
        description: String,
    ) -> Self {
        Self { id, username, discriminator, last_check_in, picture, account_creation, description }
    }
}


