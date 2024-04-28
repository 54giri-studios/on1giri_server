use rocket::{http::{Cookie, CookieJar}, serde::json::Json};

use crate::{ErrorResponse, JsonResponse};

#[get("/logout")]
pub async fn logout<'a>(cookies: &CookieJar<'a>) {
    cookies.remove_private("token");
}

