use chrono::{Duration, Local};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::header::HeaderValue;

use crate::{config::config, errors::ServiceError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub iat: i64,
    pub exp: i64,
    pub subject: Uuid,
}

impl JwtClaims {
    pub fn from_user_id(id: &Uuid) -> Self {
        Self {
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
            subject: id.clone(),
        }
    }

    pub fn sign(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let cfg = config();

        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(cfg.jwt_secret.as_ref()),
        )
    }
}

pub fn verify_token(auth_header: &HeaderValue) -> Result<Uuid, ServiceError> {
    let auth_str = auth_header
        .to_str()
        .map_err(|_| ServiceError::Unauthorized)?;

    if !auth_str.starts_with("Bearer ") {
        return Err(ServiceError::Unauthorized);
    }

    let token = &auth_str[7..];

    let cfg = config();

    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(cfg.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| ServiceError::Unauthorized)?;

    Ok(token_data.claims.subject)
}
