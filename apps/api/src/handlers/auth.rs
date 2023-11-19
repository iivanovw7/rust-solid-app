use crate::auth::{create_jwt, hash, PrivateClaim};
use crate::database::Pool;
use crate::errors::ApiError;
use crate::handlers::user::UserResponse;
use crate::helpers::{respond_json, respond_ok};
use crate::models::user::find_by_auth;
use crate::validate::validate;
use actix_identity::Identity;
#[allow(unused_imports)]
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{block, Data, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator_derive::Validate;

#[derive(Clone, ToSchema, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[schema(example = "test@email.com")]
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[schema(example = "xxxxxxxx")]
    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = StatusCode::OK, description = "Login user.", body = UserResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
    )
)]
#[post("/login")]
pub async fn login(
    id: Identity,
    pool: Data<Pool>,
    body: Json<LoginRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&body)?;

    let hashed = hash(&body.password);
    let user = block(move || find_by_auth(&pool, &body.email, &hashed))
        .await?
        .unwrap();

    let private_claim = PrivateClaim::new(user.id, user.email.clone());
    let jwt = create_jwt(private_claim)?;

    id.remember(jwt);
    respond_json(user.into())
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/logout",
    responses(
        (status = StatusCode::OK, description = "Logout user."),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
    )
)]
#[get("/logout")]
pub async fn logout(id: Identity) -> Result<HttpResponse, ApiError> {
    id.forget();
    respond_ok()
}
