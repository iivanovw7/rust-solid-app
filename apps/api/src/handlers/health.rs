use crate::errors::ApiError;
use crate::helpers::respond_json;
#[allow(unused_imports)]
use actix_web::{get, http::StatusCode, web::Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Serialize, PartialEq, Debug, Clone)]
pub struct HealthResponse {
    #[schema(example = "ok")]
    pub status: String,
    #[schema(example = "1.0.1")]
    pub version: String,
}

#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = StatusCode::OK, description = "Check api status.", body = HealthResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
    )
)]
#[get("/api/health")]
pub async fn get_health() -> Result<Json<HealthResponse>, ApiError> {
    respond_json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}
