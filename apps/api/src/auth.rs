use crate::config::CONFIG;
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};

pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    let domain: String = std::env::var(&CONFIG.domain).unwrap_or_else(|_| "localhost".to_string());

    IdentityService::new(
        CookieIdentityPolicy::new(CONFIG.session_key.as_ref())
            .name(&CONFIG.session_name)
            .path(&CONFIG.session_path)
            .domain(domain.as_str())
            .max_age(time::Duration::minutes(CONFIG.session_timeout))
            .secure(CONFIG.session_secure),
    )
}

pub fn get_cors_service() -> Cors {
    Cors::default()
        .supports_credentials()
        .allowed_origin("http://127.0.0.1:3000")
        .allowed_origin("http://localhost:3000")
        .allowed_origin("https://127.0.0.1:3000")
        .allowed_origin("https://localhost:3000")
        .allowed_origin(&CONFIG.domain)
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}
