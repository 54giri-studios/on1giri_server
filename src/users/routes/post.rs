use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::{serde::json::Json, State};

use crate::{schema::users, NewUser, DbPool, User};

#[post("/create", format = "json", data = "<new_user>")]
pub async fn user_create<'a>(pool: &State<DbPool>, new_user: Json<NewUser<'a>>) -> Json<User> {
    let mut conn = pool.get().await.unwrap();

    let new_user = new_user.into_inner();

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    user.into()
}

/*

#[post("/login", data = "<credentials>")]
pub async fn user_login2<'a>(
    pool: &State<DbPool>, 
    system_random: &State<SystemRandom>,
    credentials: Form<LoginForm<'a>>, 
    cookies: &CookieJar<'a>) -> Response<'a> {

    let token_value = {
        let now = chrono::Utc::now();
        let username = credentials.email;
        let gibberish = {
            let mut buffer = [0_u8; 8];
            system_random.fill(&mut buffer).expect("Failed to generate key");
            buffer
        };
    };
    
    let token = Cookie::build("token");
}
*/
