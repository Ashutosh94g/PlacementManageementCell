use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse,
};

use futures_util::future::LocalBoxFuture;

use crate::utils::decode_jwt;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        // println!("Hi from start. You requested: {}", req.path());

        // let fut = self.service.call(req);

        // Box::pin(async move {
        //     let res = fut.await?;

        //     println!("Hi from response");
        //     Ok(res)
        // })

        let mut token_verified = false;
        if req.uri().path() == "/api/register" && req.method() == "POST"
            || req.uri().path() == "/api/login" && req.method() == "POST"
        {
            token_verified = true;
        }
        if let Some(t) = req.headers_mut().get("AUTHORIZATION") {
            if let Ok(token) = t.to_str() {
                if token.starts_with("bearer") || token.starts_with("Bearer") {
                    let token = token[6..].trim();
                    if let Ok(data) = decode_jwt(token.to_owned()) {
                        req.headers_mut().insert(
                            header::HeaderName::from_static("user_email"),
                            header::HeaderValue::from_str(&data.claims.email).unwrap(),
                        );

                        token_verified = true;
                    }
                }
            }
        }
        if token_verified {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({ "error": "Unauthorized", "msg": "please login" }));
            Box::pin(async move { Ok(req.into_response(response)) })
        }
    }
}
