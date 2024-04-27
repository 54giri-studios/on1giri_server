use diesel::prelude::*;

const EVERYONE: &str = "everyone";
const OWNER: &str = "owner";
const STANDARD: &str = "standard";

#[derive(Debug, Insertable, Queryable, QueryableByName)]
#[diesel(table_name = crate::schema::roles_category)]
pub struct RoleCategory {
    category: &'static str,
}

impl RoleCategory {

    pub fn everyone() -> Self {
        Self { category: &EVERYONE }
    }

    pub fn owner() -> Self {
        Self { category: &OWNER }
    }

    pub fn standard() -> Self {
        Self { category: &STANDARD }
    }
}

impl ToString for RoleCategory {
    fn to_string(&self) -> String {
        return self.category.to_string();
    }
}