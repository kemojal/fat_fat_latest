use axum::extract::Path;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::payment_handlers::{
    cancel_payment, delete_payment, get_merchant_payments, get_my_payments, make_payment,
    update_payment,
};


pub fn payment_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
    
    Router::new()
        .route("/:merchant_id/payments", get(get_merchant_payments))
        .route("/my/:user_id/payments", get(get_my_payments))
        .route("/create", post(make_payment))
        .route("/:payment_id/update", put(update_payment))
        .route("/:payment_id/delete", delete(delete_payment))
        .route("/:payment_id/cancel", put(cancel_payment))
}
