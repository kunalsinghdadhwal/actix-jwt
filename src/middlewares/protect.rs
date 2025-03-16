use actix_web::{
    Error as ActixWebError, FromRequest, HttpRequest,
    dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use std::future::{Ready, ready};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationBody {
    token: String,
}

pub struct Protected;

impl<S, B> Transform<S, ServiceRequest> for Protected
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type InitError = ();
    type Transform = ProtectedMiddlewre<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ProtectedMiddlewre { service }))
    }
}

pub struct ProtectedMiddlewre<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ProtectedMiddlewre<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from a start. You Requested: {}", req.path());
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            println!("Hi from a response");
            Ok(res)
        })
    }
}
