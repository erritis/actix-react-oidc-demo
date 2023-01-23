use actix_web::{get, HttpResponse};
use aliri_actix::scope_policy;


#[get("/healthcheck")]
async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("I'm alive!")
}



scope_policy!(Profile / ProfileScope; "profile");


#[get("/userinfo")]
pub async fn userinfo(user: Profile) -> HttpResponse {
    HttpResponse::Ok().json(user.claims())
}