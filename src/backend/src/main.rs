
mod endpoints;

use actix_4_jwt_auth::{Oidc, OidcBiscuitValidator, biscuit::{ValidationOptions, Validation}, OidcConfig};
use actix_web::{middleware::Logger, App, HttpServer, web};
use actix_cors::Cors;

use endpoints::{healthcheck, userinfo};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");

    let oidc = Oidc::new(OidcConfig::Issuer(authority.clone().into())).await.unwrap();

    let biscuit_validator = OidcBiscuitValidator { options: ValidationOptions {
            issuer: Validation::Validate(authority),
            ..ValidationOptions::default()
        }
    };
    //let scope_validator = OidcScopeValidator(vec!["openid", "profile"]);

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().supports_credentials();

        App::new()
           
            .wrap(Logger::default())
            .service(
                web::scope("")
                .app_data(oidc.clone())
                //.wrap(scope_validator.clone())
                .wrap(biscuit_validator.clone())
                //.wrap(OidcBiscuitValidator::default())
                .wrap(cors)
                .service(userinfo)
            )
            .service(healthcheck)
    })
    .bind(("0.0.0.0", 52635))?
    .run()
    .await
}