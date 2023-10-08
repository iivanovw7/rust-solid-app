use crate::handlers::health::get_health;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .service(get_health);
}
