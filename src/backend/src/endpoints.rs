use actix_4_jwt_auth::{AuthenticatedUser};
use actix_web::{get, HttpResponse};
use serde::{Serialize, Deserialize};


#[get("/healthcheck")]
async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("I'm alive!")
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FoundClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub name: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub scope: String,
}


#[get("/userinfo")]
pub async fn userinfo(user: AuthenticatedUser<FoundClaims>) -> HttpResponse {
    HttpResponse::Ok().json(user.claims)
}