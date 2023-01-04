
mod auth;
mod endpoints;

use actix_4_jwt_auth::{OIDCValidator, OIDCValidatorConfig};
use biscuit::{ValidationOptions};
use actix_web::{middleware::Logger, App, HttpServer, web};
use actix_cors::Cors;

use endpoints::{healthcheck, userinfo};
use auth::{AuthScope};






#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");


    let validation_options = ValidationOptions::default(); 
     
    let created_validator = OIDCValidator::new_from_issuer(authority.clone(), validation_options).await.unwrap();
    let validator_config = OIDCValidatorConfig {
        issuer: authority,
        validator: created_validator,
    };
    let auth_scope = AuthScope(vec!["profile"]);


    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("")
                .app_data(validator_config.clone())
                .wrap(auth_scope.clone())
                .service(userinfo)
            )
            .service(healthcheck)
    })
    .bind(("0.0.0.0", 52635))?
    .run()
    .await
}