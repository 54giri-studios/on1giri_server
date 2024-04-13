use diesel::{prelude::*, sql_types::Date};
use std::borrow::Cow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel<'a> {
    pub id: i32,
    pub guild_id: i32,
    pub name: Cow<'a, str>,
    pub kind: Cow<'a, str>
}

#[derive(Debug, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::channel_kinds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelKind<'a> {
    kind: Cow<'a, str>
}

impl<'a> ChannelKind<'a> {
    pub fn new(kind: &'a str) -> Self {
        Self { kind: kind.into() }
    }

    pub fn text() -> Self {
        Self::new("text")
    }

    pub fn category() -> Self {
        Self::new("category")
    }

    pub fn voice() -> Self {
        Self::new("voice")
    }

    pub fn system() -> Self {
        Self::new("system")
    }
}

#[derive(Debug)]
pub struct HistoryConfig {
    pub limit: Option<i32>,
    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>
}
