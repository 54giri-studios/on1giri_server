use std::borrow::Cow;

use chrono::{DateTime, Utc};
use diesel::{deserialize::FromSqlRow, prelude::Insertable, Selectable};
use rocket::{data::{self, FromData}, Request, Data};

use crate::User;

#[derive(Serialize, Deserialize, Debug, Selectable, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message<'a> {
    id: i32,
    channel_id: i32,
    author_id: i32,
    content: Cow<'a, str>,
    creation_date: DateTime<Utc>
}