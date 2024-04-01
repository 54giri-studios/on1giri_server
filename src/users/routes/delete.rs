//! Functions / routes used to delete users

/// Deletes an user from its id
/// 
/// # Arguments
/// * `id` - The user's unique identifier
#[delete("/<id>")]
pub async fn delete_by_id(id: i64) {
    todo!()
}

/// Deletes an user using it's name and discriminator
/// 
/// # Arguments
/// * `username` - The user's current username
/// * `discriminator` - The user's current discriminator, must be between 0 and 9999 included
#[delete("/<username>/<discriminator>")]
pub async fn delete_by_username_discriminator(username: &str, discriminator: u8) {
    todo!()
}

