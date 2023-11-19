use crate::database::Pool;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::user::{create, delete, find, get_all, update, NewUser, UpdateUser, User};
use crate::validate::validate;
#[allow(unused_imports)]
use actix_web::{
    get, patch, post,
    web::{block, Data, Json, Path},
    HttpResponse, Result,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator_derive::Validate;

#[derive(Debug, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    #[schema(value_type = String, example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "Name")]
    pub first_name: String,
    #[schema(example = "Last Name")]
    pub last_name: String,
    #[schema(example = "test@test.com")]
    pub email: String,
    #[schema(example = "user")]
    pub role: String,
}

#[derive(Debug, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct UsersResponse(pub Vec<UserResponse>);

#[derive(Clone, Debug, ToSchema, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    #[schema(example = "Name")]
    #[validate(length(
        min = 3,
        message = "first_name is required and must be at least 3 characters"
    ))]
    pub first_name: String,

    #[schema(example = "Last Name")]
    #[validate(length(
        min = 3,
        message = "last_name is required and must be at least 3 characters"
    ))]
    pub last_name: String,

    #[schema(example = "test@email.com")]
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[schema(example = "xxxxxxx")]
    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

#[derive(Clone, Debug, ToSchema, Deserialize, Serialize, Validate)]
pub struct UpdateUserRequest {
    #[schema(example = "Name")]
    #[validate(length(
        min = 3,
        message = "first_name is required and must be at least 3 characters"
    ))]
    pub first_name: String,

    #[schema(example = "Last Name")]
    #[validate(length(
        min = 3,
        message = "last_name is required and must be at least 3 characters"
    ))]
    pub last_name: String,

    #[schema(example = "test@email.com")]
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/user/get-user/{id}",
    responses(
        (status = StatusCode::OK, description = "Get user data.", body = UserResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
    )
)]
#[get("/get-user")]
pub async fn get_user(
    user_id: Path<Uuid>,
    pool: Data<Pool>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = block(move || find(&pool, *user_id)).await?;

    respond_json(user?)
}

#[utoipa::path(
    get,
    path = "/api/v1/user/get-users",
    responses(
        (status = StatusCode::OK, description = "Get users data.", body = UsersResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
    )
)]
#[get("/get-users")]
pub async fn get_users(pool: Data<Pool>) -> Result<Json<UsersResponse>, ApiError> {
    let users = block(move || get_all(&pool)).await?;

    respond_json(users?)
}

#[utoipa::path(
    post,
    path = "/api/v1/user/create-user",
    request_body = CreateUserRequest,
    responses(
        (status = StatusCode::OK, description = "Create new user.", body = UsersResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
    )
)]
#[post("/create-user")]
pub async fn create_user(
    pool: Data<Pool>,
    body: Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&body)?;

    let user_id = Uuid::new_v4();
    let new_user: User = NewUser {
        id: user_id.to_string(),
        first_name: body.first_name.to_string(),
        last_name: body.last_name.to_string(),
        email: body.email.to_string(),
        password: body.password.to_string(),
        created_by: user_id.to_string(),
        updated_by: user_id.to_string(),
    }
    .into();

    let user = block(move || create(&pool, &new_user)).await?;

    respond_json(user?.into())
}

#[utoipa::path(
    patch,
    path = "/api/v1/user/update-user/{id}",
    request_body = UpdateUserRequest,
    responses(
        (status = StatusCode::OK, description = "Update user", body = UsersResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
    )
)]
#[patch("/update-user/{id}")]
pub async fn update_user(
    user_id: Path<Uuid>,
    pool: Data<Pool>,
    body: Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&body)?;

    let update_user = UpdateUser {
        id: user_id.to_string(),
        first_name: body.first_name.to_string(),
        last_name: body.last_name.to_string(),
        email: body.email.to_string(),
        updated_by: user_id.to_string(),
    };

    let user = block(move || update(&pool, &update_user)).await?;

    respond_json(user?.into())
}

#[utoipa::path(
    delete,
    path = "/api/v1/user/delete-user/{id}",
    responses(
        (status = StatusCode::OK, description = "Update user"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
    )
)]
#[patch("/delete-user/{id}")]
pub async fn delete_user(user_id: Path<Uuid>, pool: Data<Pool>) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *user_id)).await?;
    respond_ok()
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: Uuid::parse_str(&user.id).unwrap(),
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
            email: user.email.to_string(),
            role: user.role.to_string(),
        }
    }
}

impl From<Vec<User>> for UsersResponse {
    fn from(users: Vec<User>) -> Self {
        UsersResponse(users.into_par_iter().map(|user| user.into()).collect())
    }
}
