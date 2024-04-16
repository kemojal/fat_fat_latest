// use axum::{
//     body::Body,
//     http::{header, Response, StatusCode},
//     middleware::{Next, RequestParts},
//     response::IntoResponse,
// };
// use serde::{Deserialize, Serialize};
// use std::{
//     collections::HashMap,
//     convert::{Infallible, TryFrom},
// };
// use tracing::{error, info};

// #[derive(Debug, Deserialize, Serialize)]
// struct ErrorResponse {
//     code: u16,
//     message: String,
// }

// pub async fn error_handler_middleware(
//     mut req: RequestParts<Body>,
//     next: Next<Body>,
// ) -> Result<impl IntoResponse, Infallible> {
//     let res = next.run(req).await;

//     match res {
//         Ok(response) => Ok(response),
//         Err(err) => {
//             error!("Error handling request: {:?}", err);

//             let (status, message) = match err {
//                 axum::Error::JsonError(e) => {
//                     (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e))
//                 }
//                 axum::Error::TypeMismatch(_, _) => {
//                     (StatusCode::BAD_REQUEST, "Invalid request data".to_string())
//                 }
//                 axum::Error::InvalidRoute => {
//                     (StatusCode::NOT_FOUND, "Resource not found".to_string())
//                 }
//                 axum::Error::ServerError(e) => {
//                     (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal server error: {}", e))
//                 }
//                 _ => {
//                     (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".to_string())
//                 }
//             };

//             let error_response = ErrorResponse {
//                 code: status.as_u16(),
//                 message,
//             };

//             Ok(Response::builder()
//                 .status(status)
//                 .header(header::CONTENT_TYPE, "application/json")
//                 .body(AxumJson(error_response).into_response().0)
//                 .unwrap())
//         }
//     }
// }