use crate::auth::{decode_jwt, PrivateClaim};
use crate::errors::ApiError;
use actix_identity::RequestIdentity;
#[allow(unused_imports)]
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ready, LocalBoxFuture, Ready};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
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

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let identity = RequestIdentity::get_identity(&request).unwrap_or("".into());
        let private_claim: Result<PrivateClaim, ApiError> = decode_jwt(&identity);
        let is_logged_in = private_claim.is_ok();
        let unauthorized = !is_logged_in && request.path() != "/api/v1/auth/login";

        if unauthorized {
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::Unauthorized().finish().map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let response = self.service.call(request);

        Box::pin(async move { response.await.map(ServiceResponse::map_into_left_body) })
    }
}
