use crate::errors::ApiError;
use crate::handlers::{auth, health, user};
use actix_web::web;
use log::info;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn get_swagger_service(cfg: &mut web::ServiceConfig) {
    let args: Vec<String> = env::args().collect();
    let arg = &args[0];

    if arg.contains("release") {
        return;
    }

    #[derive(OpenApi)]
    #[openapi(
        paths(
            health::get_health,
            auth::login,
            auth::logout,
            user::get_user,
            user::get_users,
            user::create_user,
            user::update_user,
            user::delete_user,
        ),
        components(
            schemas(health::HealthResponse, ApiError),
            schemas(auth::LoginRequest, ApiError),
            schemas(user::UserResponse, ApiError),
            schemas(user::UsersResponse, ApiError),
            schemas(user::CreateUserRequest, ApiError),
            schemas(user::UpdateUserRequest, ApiError),
        ),
        tags(
            (name = "health"),
            (name = "auth"),
            (name = "user")
        ),
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    info!("starting swagger-ui");

    cfg.service(
        SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/api-docs/openapi.json", openapi.clone()),
    );
}
