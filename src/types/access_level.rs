use diesel::prelude::*;

/// Represents how much an user can do
/// Mirrors [crate::schema::access_levels]
#[derive(Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::access_levels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccessLevel {
    level: String
}

impl AccessLevel {
    /// A superuser that has unrestricted access to all endpoints
    pub fn admin() -> Self {
        Self { level: "admin".into() }
    }

    /// A regular user that has regulated access to endpoints
    pub fn regular() -> Self {
        Self { level: "regular".into() }
    }
}

impl ToString for AccessLevel {
    fn to_string(&self) -> String {
        self.level.clone()
    }
}