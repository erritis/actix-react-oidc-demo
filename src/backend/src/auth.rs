
use std::{future::{ready, Ready}, rc::Rc};

use actix_4_jwt_auth::AuthenticatedUser;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use serde::{Serialize, Deserialize};

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

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Debug, PartialEq, Clone)]
pub struct AuthScope(pub Vec<&'static str>);

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthScope
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthScopeMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthScopeMiddleware { service: Rc::new(service), scopes: self.0.clone() }))
    }
}

pub struct AuthScopeMiddleware<S> {
    service: Rc<S>,
    scopes: Vec<&'static str>,
}

impl<S, B> Service<ServiceRequest> for AuthScopeMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let scopes = self.scopes.clone();

        Box::pin(async move {
            let user: AuthenticatedUser<FoundClaims> = req.extract::<AuthenticatedUser<FoundClaims>>().await?;
            let res =  scopes.iter().all(|item| user.claims.scope.contains(item));
            if res == true {
                let fut = svc.call(req);
                let res = fut.await?;
                Ok(res)
            } else {
                Err(actix_web::error::ErrorForbidden("Scopes in an access token are not valid.").into())
            }
        })
    }
}


