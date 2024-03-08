use actix_web::{error::InternalError, Error, HttpResponse, HttpMessage, dev::{ServiceResponse, Transform, Service}};
use std::env;
use jsonwebtoken::{decode, Validation, Algorithm, DecodingKey};
use serde::{Serialize, Deserialize};
use futures::future::{ok, Ready, Future};
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    id: Option<i32>,
    email: Option<String>,
    username: Option<String>,
    security_level: Option<i16>,
    employee_id: Option<i32>,
    sub: String,
    exp: usize,
}

pub struct JWTAuth;

impl<S, B, Req> Transform<S, Req> for JWTAuth
where
    S: Service<Req, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    Req: HttpMessage + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<actix_web::Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B, Req: HttpMessage + 'static> Service<Req> for AuthMiddleware<S>
where
    S: Service<Req, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = actix_web::Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<actix_web::Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: Req) -> Self::Future  {
        dotenv::dotenv().ok();

        let skip_auth_str = env::var("SKIP_AUTH").expect("SKIP_AUTH is not set");

        let skip_auth: bool = skip_auth_str.parse().unwrap_or_else(|_| {
            panic!("SKIP_AUTH should be a boolean and set to either 'true' or 'false'")
        });

        let should_skip: bool = {
            if let Some(uri) = req.extensions().get::<String>() {
                skip_auth || uri.contains("/api/v1/auth")
            } else {
                false
            }
        };

        if should_skip {
            return Box::pin(self.service.call(req));
        }

        let headers = req.headers().clone();

        let fut = self.service.call(req);

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set");

        Box::pin(async move {
            if let Some(auth_header) = headers.get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];

                        let decoding_key_result = DecodingKey::from_base64_secret(&jwt_secret);
                        
                        match decoding_key_result {
                            Ok(decoding_key) => {
                                let validation = Validation::new(Algorithm::HS256);
                                match decode::<Claims>(token, &decoding_key, &validation) {
                                    Ok(_) => {
                                        return fut.await;
                                    },
                                    Err(_) => {
                                        return Err(InternalError::from_response("UNAUTHORIZED", HttpResponse::Unauthorized().finish()).into());
                                    }
                                }
                            },
                            Err(_) => {
                                // Redirect to login route if decoding_key is an error
                                return Err(InternalError::from_response("UNAUTHORIZED", HttpResponse::Found().append_header(("Location", "/api/v1/auth")).finish()).into());
                            }
                        }
                    }
                }
            }
            Err(InternalError::from_response("UNAUTHORIZED", HttpResponse::Unauthorized().finish()).into())
        })
    }
}