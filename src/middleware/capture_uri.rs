use actix_web::dev::{Transform, Service, ServiceResponse, ServiceRequest};
use actix_web::HttpMessage;
use actix_web::Result;
use core::future::Ready;
use actix_web::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct CaptureUriMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for CaptureUriMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let uri = req.uri().to_string();
        req.extensions_mut().insert(uri);
        Box::pin(self.service.call(req))
    }
}

pub struct CaptureUri;

impl<S> Transform<S, ServiceRequest> for CaptureUri
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = CaptureUriMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(CaptureUriMiddleware { service }))
    }
}