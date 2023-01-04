use actix_4_jwt_auth::AuthenticatedUser;
use actix_web::{get, HttpResponse};
use serde::{Serialize};

use crate::auth::FoundClaims;

#[get("/healthcheck")]
async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("I'm alive!")
}


#[derive(Debug, Serialize)]
pub struct Forecast<'a>{
    summary: &'static str,
    temperaturec: i8,
    date: &'a str
}


#[get("/userinfo")]
pub async fn userinfo(user: AuthenticatedUser<FoundClaims>) -> HttpResponse {
    HttpResponse::Ok().json(user.claims)
}