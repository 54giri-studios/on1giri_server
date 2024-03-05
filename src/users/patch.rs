//! Functions / routes used to update informations about users

/// Updates an user from its id
/// 
/// # Arguments
/// * `id` - The user's unique identifier
#[patch("/<id>/<field>/<value>")]
pub async fn update_by_id(id: i64, field: &str, value: &str) {
    todo!()
}

/// Updates an user using it's name and discriminator
/// 
/// # Arguments
/// * `username` - The user's current username
/// * `discriminator` - The user's current discriminator, must be between 0 and 9999 included
#[patch("/<username>/<discriminator>/<field>/<value>")]
pub async fn update_by_username_discriminator(
    username: &str, 
    discriminator: u8,
    field: &str,
    value: &str
) {
}

