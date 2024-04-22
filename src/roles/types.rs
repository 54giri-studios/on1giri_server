use diesel::{deserialize::{Queryable, QueryableByName}, prelude::Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, QueryableByName)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    id: i32,
    guild_id: i32,
    name: String
}
