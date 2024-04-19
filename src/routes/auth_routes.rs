use axum::{Json, Router};
use std::sync::Arc;

use crate::handlers::auth_handlers::{sign_in, sign_out};

use axum::routing::post;
use sqlx::PgPool;

pub fn auth_routes(_pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
    Router::new()
        .route("/signin", post(sign_in))
        .route("/signout", post(sign_out))
}
