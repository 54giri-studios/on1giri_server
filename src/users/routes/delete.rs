//! Functions / routes used to delete users

/// Deletes an user from its id
/// 
/// # Arguments
/// * `id` - The user's unique identifier
#[delete("/<id>")]
pub async fn delete_by_id(id: i64) {
    todo!()
}
