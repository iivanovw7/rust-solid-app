use crate::config::CONFIG;
use crate::errors::ApiError;
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.auth_salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrivateClaim {
    pub user_id: Uuid,
    pub email: String,
    exp: i64,
}

impl PrivateClaim {
    pub fn new(user_id: Uuid, email: String) -> Self {
        Self {
            user_id,
            email,
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

pub fn create_jwt(private_claim: PrivateClaim) -> Result<String, ApiError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());

    encode(&Header::default(), &private_claim, &encoding_key)
        .map_err(|e| ApiError::CannotEncodeJwtToken(e.to_string()))
}

pub fn decode_jwt(token: &str) -> Result<PrivateClaim, ApiError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());

    decode::<PrivateClaim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ApiError::CannotDecodeJwtToken(e.to_string()))
}
