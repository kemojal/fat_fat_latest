use std::sync::Arc;
use axum::{Json, Router};

use axum::routing::post;
use sqlx::PgPool;
use crate::handlers::auth_handlers::{sign_in, sign_out};
use crate::models::auth_models::SignInData;

pub fn auth_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>>  {


    let authPool = Arc::clone(&pool);
    let signOutPool = Arc::clone(&pool);

    Router::new()
        .route("/signin", post(move |Json(sign_in_data): Json<SignInData>| {
            sign_in(axum::Json(sign_in_data), authPool)
        }))
        .route("/signout", post(move |Json(sign_out_data): Json<SignInData>| {
            sign_out(axum::Json(sign_out_data), signOutPool)
        }))

}