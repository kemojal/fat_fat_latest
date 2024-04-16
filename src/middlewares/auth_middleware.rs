use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
  body::{Body, Bytes},
  extract::Request,
  http::{self, header, StatusCode},
  middleware::Next,
  response::{IntoResponse, Response},
};
use colored::Colorize;
use http_body_util::BodyExt;
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use tracing::{error, info, trace};

use super::logger_middleware::buffer_and_print;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
  pub email: String,
  pub verified: bool,
  // pub created_at: NaiveDateTime,
  // pub updated_at: NaiveDateTime,
  pub username: String,
  pub phone_number: String,
  pub user_id: i32,
  pub exp: usize,
  // Add any other claims you want to extract
}

pub async fn auth_middleware(
  mut req: Request<Body>,
  next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
  let (parts, body) = req.into_parts();
  let bytes = buffer_and_print("request", body).await?;

  if !has_valid_jwt_token(&parts.headers) {
    return Err((
      StatusCode::UNAUTHORIZED,
      "Missing or invalid Authorization header".to_string(),
    ));
  }

  let jwt_token = match get_jwt_token(&parts.headers) {
    Some(token) => token,
    None => {
      return Err((
        StatusCode::UNAUTHORIZED,
        "Missing or invalid Authorization header".to_string(),
      ))
    }
  };

  let jwt_payload = match decode_jwt_token(&jwt_token) {
    Ok(payload) => payload,
    Err(err) => {
      error!("Error decoding JWT token: {:?}", err);
      return Err((
        StatusCode::UNAUTHORIZED,
        "Invalid Authorization header".to_string(),
      ));
    }
  };

  if is_token_expired(&jwt_payload) {
    return Err((
      StatusCode::UNAUTHORIZED,
      "JWT token has expired".to_string(),
    ));
  }

  let req = Request::from_parts(parts, Body::from(bytes));
  let res = next.run(req).await;
  // Store the JWT payload in the request extensions
 

  Ok(res)
}

pub fn has_valid_jwt_token(headers: &http::HeaderMap) -> bool {
  headers
    .get(header::AUTHORIZATION)
    .and_then(|value| value.to_str().ok())
    .map(|auth_header| auth_header.starts_with("Bearer "))
    .unwrap_or(false)
}

pub fn get_jwt_token(headers: &http::HeaderMap) -> Option<String> {
  headers
    .get(header::AUTHORIZATION)
    .and_then(|value| value.to_str().ok())
    .filter(|auth_header| auth_header.starts_with("Bearer "))
    .map(|auth_header| auth_header["Bearer ".len()..].trim().to_string())
}

pub fn decode_jwt_token(token: &str) -> Result<JwtPayload, jsonwebtoken::errors::Error> {
  let validation = Validation::default();
  let key = "CfLTk9J0MA3jBF3/zuE4VUyN7djM2KMPy4otUpbkbE8="; // Replace with your actual JWT secret key
  let payload = decode::<JwtPayload>(
    token,
    &jsonwebtoken::DecodingKey::from_secret(key.as_ref()),
    &validation,
  )?
  .claims;
  Ok(payload)
}

pub fn is_token_expired(jwt_payload: &JwtPayload) -> bool {
  let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as usize;
  jwt_payload.exp < now
}
