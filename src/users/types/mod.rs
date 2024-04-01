#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::users::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    password: String,
    user_type: i32,
    email: String
}
