// use axum::{
//     extract::RequestExt,
//     http::{StatusCode, header},
//     middleware::Next,
//     response::Response,
// };
// use std::{
//     collections::HashMap,
//     sync::{Arc, Mutex},
//     time::Duration,
// };
// use tokio::time::Instant;

// #[derive(Debug, Clone)]
// struct RateLimitInfo {
//     limit: usize,
//     remaining: usize,
//     reset_at: Instant,
// }

// #[derive(Debug, Clone)]
// pub struct RateLimitStore {
//     rate_limits: Arc<Mutex<HashMap<String, RateLimitInfo>>>,
// }

// impl RateLimitStore {
//     pub fn new() -> Self {
//         Self {
//             rate_limits: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }

//     async fn check_and_update_rate_limit(
//         &self,
//         key: &str,
//         limit: usize,
//         period: Duration,
//     ) -> Result<(), (StatusCode, String)> {
//         let mut rate_limits = self.rate_limits.lock().unwrap();

//         let now = Instant::now();
//         let rate_limit = rate_limits
//             .entry(key.to_owned())
//             .or_insert_with(|| RateLimitInfo {
//                 limit,
//                 remaining: limit,
//                 reset_at: now + period,
//             });

//         if rate_limit.remaining > 0 {
//             rate_limit.remaining -= 1;
//             Ok(())
//         } else if now < rate_limit.reset_at {
//             let reset_in = (rate_limit.reset_at - now).as_secs() as usize;
//             Err((
//                 StatusCode::TOO_MANY_REQUESTS,
//                 format!("Rate limit exceeded, try again in {} seconds", reset_in),
//             ))
//         } else {
//             rate_limit.remaining = limit - 1;
//             rate_limit.reset_at = now + period;
//             Ok(())
//         }
//     }
// }

// pub async fn rate_limit_middleware(
//     req: axum::http::Request<axum::body::Body>,
//     next: Next,
//     rate_limit_store: axum::extract::StateExtractor<RateLimitStore>,
// ) -> Result<Response, (StatusCode, String)> {
//     let key = req
//         .peer_addr()
//         .map(|addr| addr.to_string())
//         .unwrap_or_else(|| "anonymous".to_string());
//     let limit = 10;
//     let period = Duration::from_secs(60);

//     rate_limit_store
//         .check_and_update_rate_limit(&key, limit, period)
//         .await?;

//     let res = next.run(req).await;

//     let (mut parts, body) = res.into_parts();
//     parts.headers.insert(
//         header::X_RateLimit_Limit,
//         limit.to_string().parse().unwrap(),
//     );
//     parts.headers.insert(
//         header::X_RateLimit_Remaining,
//         rate_limit_store
//             .rate_limits
//             .lock()
//             .unwrap()
//             .get(&key)
//             .map(|info| info.remaining.to_string())
//             .unwrap_or_else(|| "".to_string())
//             .parse()
//             .unwrap(),
//     );
//     parts.headers.insert(
//         header::X_RateLimit_Reset,
//         rate_limit_store
//             .rate_limits
//             .lock()
//             .unwrap()
//             .get(&key)
//             .map(|info| info.reset_at.elapsed().as_secs().to_string())
//             .unwrap_or_else(|| "".to_string())
//             .parse()
//             .unwrap(),
//     );

//     Ok(Response::from_parts(parts, body))
// }