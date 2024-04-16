
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
use uuid::Uuid;



#[derive(Debug, Serialize, Deserialize)]
struct JwtPayload {
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

pub async fn logger_middleware(
  req: Request,
  next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
  let (parts, body) = req.into_parts();


  // Generate a unique request identifier
  let request_id = Uuid::new_v4().to_string();


//   if !has_valid_jwt_token(&parts.headers) {
//     return Err((
//         StatusCode::UNAUTHORIZED,
//         "Missing or invalid Authorization header".to_string(),
//     ));
// }

// let jwt_token = match get_jwt_token(&parts.headers) {
//     Some(token) => token,
//     None => {
//         return Err((
//             StatusCode::UNAUTHORIZED,
//             "Missing or invalid Authorization header".to_string(),
//         ))
//     }
// };





  let bytes = buffer_and_print("request", body).await?;
  // Decode the JWT token and extract the payload
//   let jwt_payload = match decode_jwt_token(&jwt_token) {
//     Ok(payload) => payload,
//     Err(err) => {
//         error!("Error decoding JWT token: {:?}", err);
//         return Err((
//             StatusCode::UNAUTHORIZED,
//             "Invalid Authorization header".to_string(),
//         ));
//     }
// };

// // Check if the token has expired
// if is_token_expired(&jwt_payload) {
//     return Err((
//         StatusCode::UNAUTHORIZED,
//         "JWT token has expired".to_string(),
//     ));
// }



  print_request_info(&parts,
    &request_id
    // , &jwt_payload
);

  tracing::info!(
      target: "request ➡️",
      method = %parts.method,
      uri = %parts.uri,
      version = ?parts.version,
      headers = ?parts.headers,
      "request received"
  );

  //   tracing::info!(target: "request ➡️", method = ?parts.method, uri = ?parts.uri, ddd = ?parts.version, body = ?std::str::from_utf8(&bytes), "request received/n");

  let req = Request::from_parts(parts, Body::from(bytes));

  let res = next.run(req).await;

  let (parts, body) = res.into_parts();
  let bytes = buffer_and_print("response", body).await?;

  //   tracing::info!(target: "🚥 response",  status = ?parts.status, version= ?parts.version, body = ?std::str::from_utf8(&bytes), "response sent");

  print_response_info(&parts, &request_id);

  let res = Response::from_parts(parts, Body::from(bytes));

  Ok(res)
}

pub async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
  B: axum::body::HttpBody<Data = Bytes>,
  B::Error: std::fmt::Display,
{
  let bytes = match body.collect().await {
    Ok(collected) => collected.to_bytes(),
    Err(err) => {
      return Err((
        StatusCode::BAD_REQUEST,
        format!("failed to read {direction} body: {err}"),
      ));
    }
  };

  if let Ok(body) = std::str::from_utf8(&bytes) {
    tracing::debug!("{direction} body = {body:?}");
  }

  Ok(bytes)
}


// fn has_valid_jwt_token(headers: &http::HeaderMap) -> bool {
//     headers
//         .get(header::AUTHORIZATION)
//         .and_then(|value| value.to_str().ok())
//         .map(|auth_header| auth_header.starts_with("Bearer "))
//         .unwrap_or(false)
// }

// fn get_jwt_token(headers: &http::HeaderMap) -> Option<String> {
//     headers
//         .get(header::AUTHORIZATION)
//         .and_then(|value| value.to_str().ok())
//         .filter(|auth_header| auth_header.starts_with("Bearer "))
//         .map(|auth_header| auth_header["Bearer ".len()..].trim().to_string())
// }

// fn is_token_expired(jwt_payload: &JwtPayload) -> bool {
//     let now = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs() as usize;
//     jwt_payload.exp < now
// }

// fn decode_jwt_token(token: &str) -> Result<JwtPayload, jsonwebtoken::errors::Error> {
//     let validation = Validation::default();
//     let key = "CfLTk9J0MA3jBF3/zuE4VUyN7djM2KMPy4otUpbkbE8="; // Replace with your actual JWT secret key
//     let payload = decode::<JwtPayload>(token, &jsonwebtoken::DecodingKey::from_secret(key.as_ref()), &validation)?
//         .claims;
//     Ok(payload)
// }






fn print_request_info(parts: &http::request::Parts,
    request_id: &str,
    // , jwt_payload: &JwtPayload
) {
  println!("{}", "-".repeat(80).truecolor(0, 255, 136));
  print!("{}", "Request ➡️➡️➡️➡️ ".bold());
  println!(
    "method = {} uri = {} version = {:?} request_id = {}",
    parts.method.to_string().bold().truecolor(0, 255, 136),
    parts.uri.to_string().bold().truecolor(0, 255, 136),
    parts.version,
    request_id
  );
//   println!(
//     "JWT Payload: username = {} expiration = {} email = {} user_id = {}",
//     jwt_payload.username, jwt_payload.exp, jwt_payload.email, jwt_payload.user_id
// );
  //   println!("{}", "-".repeat(80).truecolor(0, 255, 136));
}

fn print_response_info(parts: &http::response::Parts, request_id: &str,) {
  println!("{}", "-".repeat(80).truecolor(0, 255, 136));
  print!("{}", "Response ⬅️⬅️⬅️⬅️ ".bold());
  println!(
    "status = {} version = {:?} request_id = {}",
    parts
      .status
      .as_u16()
      .to_string()
      .bold()
      .truecolor(0, 255, 136),
    parts.version,
    request_id
  );
  println!("{}", "-".repeat(80).truecolor(0, 255, 136));
  
}
