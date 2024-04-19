use axum::routing::post;
use axum::Router;
use std::sync::Arc;

use crate::handlers::register_handlers::{
    complete_registration, create_unverified_user, verify_code,
};

use sqlx::PgPool;

// use crate::services::UserServices;

pub fn register_routes(_pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
    
    // let user_service = UserServices::new(user_pool, verify_user_pool);
    Router::new()
        .route("/create", post(create_unverified_user))
        .route("/verify", post(verify_code))
        .route("/complete_registration", post(complete_registration))
}
