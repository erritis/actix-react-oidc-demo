
mod endpoints;

use actix_web::{middleware::Logger, App, HttpServer, web};
use actix_cors::Cors;


use aliri::jwt;
use aliri_oauth2::{Authority};
use endpoints::{healthcheck, userinfo};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let issuer = std::env::var("AUTHORITY").expect("AUTHORITY must be set");

    let validator = jwt::CoreValidator::default()
            .require_issuer(jwt::Issuer::new(issuer.clone()))
            //.add_allowed_audience(jwt::Audience::new(issuer.clone()))
            ;

    let jwks_url = format!("{}/protocol/openid-connect/certs", issuer);

    let authority = Authority::new_from_url(jwks_url, validator)
    .await.expect("AUTHORITY didn't create");


    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().supports_credentials();

        App::new()
           
            .wrap(Logger::default())
            .service(
                web::scope("")
                .app_data(authority.clone())
                .wrap(cors)
                .service(userinfo)
            )
            .service(healthcheck)
    })
    .bind(("0.0.0.0", 52635))?
    .run()
    .await
}