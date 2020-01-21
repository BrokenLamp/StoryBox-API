use crate::util::token::Token;
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

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
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

impl<S, B> Service for AuthenticationMiddleware<S>
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

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = ["/auth", "/info", "/stats"]
            .iter()
            .any(|route| req.path().starts_with(route));

        let mut failure = jsonwebtoken::errors::ErrorKind::InvalidToken;

        let _: Option<()> = try {
            let auth_str = req.headers_mut().get("Authorization")?.to_str().ok()?;

            if !auth_str.starts_with("bearer") && !auth_str.starts_with("Bearer") {
                failure = jsonwebtoken::errors::ErrorKind::InvalidToken;
                None?;
            }

            let token = auth_str[6..auth_str.len()].trim();

            match Token::from_jwt(token) {
                Ok(_token) => authenticate_pass = true,
                Err(e) => failure = e.into_kind(),
            };
        };

        if authenticate_pass {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .body(format!("{{ success: false, message: \"{:?}\" }}", failure))
                        .into_body(),
                ))
            })
        }
    }
}
