use std::borrow::Cow;

use crate::roles::Role;
use crate::channels::Channel;
use chrono::{DateTime, Utc};
use diesel::{deserialize::Queryable, prelude::Insertable};

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = crate::schema::guilds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guild<'a> {
    pub id: i32,
    pub name: Cow<'a, str>,
    pub owner_id: i32,
    pub description: Cow<'a, str>,
    pub creation_date: DateTime<Utc>
}
