use crate::errors::ApiError;
use crate::handlers::health;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn get_swagger_service(cfg: &mut web::ServiceConfig) {
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

    cfg.service(
        SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/api-docs/openapi.json", openapi.clone()),
    );
}
