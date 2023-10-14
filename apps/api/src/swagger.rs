use crate::errors::ApiError;
use crate::handlers::health;
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
            health::get_health
        ),
        components(
            schemas(health::HealthResponse, ApiError)
        ),
        tags(
            (name = "health")
        ),
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    info!("starting swagger-ui");

    cfg.service(
        SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/api-docs/openapi.json", openapi.clone()),
    );
}
