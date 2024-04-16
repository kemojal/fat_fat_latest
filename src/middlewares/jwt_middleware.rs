// use axum::body::Body;
// use axum::Extension;
// use axum::http::{Request, StatusCode};
// use jsonwebtoken::{DecodingKey, Validation, decode};
// use crate::models::user_models::User;
//
// pub async fn jwt_middleware(
//     request: Request<Body>,
//     key: DecodingKey,
// ) -> Result<Request<Body>, StatusCode> {
//     // Extract and validate JWT from the Authorization header
//     if let Some(authorization) = request.headers().get("Authorization") {
//         if let Ok(token) = authorization.to_str() {
//             if let Ok(token_data) = decode::<User>(&token, &key, &Validation::default()) {
//                 // Token is valid, extract user data and attach to request extensions
//                 let user = token_data.claims;
//                 let modified_request = request.insert_extension(user);
//                 return Ok(modified_request);
//             }
//         }
//     }
//     Err(StatusCode::UNAUTHORIZED)
// }