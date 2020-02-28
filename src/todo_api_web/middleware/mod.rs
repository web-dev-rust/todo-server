use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    todo_api_web::model::http::Clients,
    todo_api::{
        core::decode_jwt,
        model::core::JwtValue,
    }
};

pub struct Authentication;

impl<S: 'static, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S: 'static, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if req.path().starts_with("/api/") {
            let data = req.app_data::<Clients>().expect("Failed to parse app_data");
            let jwt = req.headers().get("x-auth");

            match jwt {
                None => Box::pin(async move {
                    Ok::<_,actix_http::error::Error>(req.into_response(
                        HttpResponse::Unauthorized()
                        .json("{\"error\": \"x-auth is required\"}")
                        .into_body()
                    ))
                }),
                Some(token) => {
                    let decoded_jwt: JwtValue = serde_json::from_value(decode_jwt(token.to_str().unwrap())).expect("Failed to parse Jwt");
                    let valid_jwt = data.postgres.send(decoded_jwt);

                    let fut = self.service.call(req);
                    Box::pin(async move {
                        match valid_jwt.await {
                            Ok(true) => {
                                let res = fut.await?;
                                Ok(res)
                            },
                            _ => {
                                Err(Error::from(()))
                            }
                        }
                    })
                }
            }
        } else {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        }
    }
}

