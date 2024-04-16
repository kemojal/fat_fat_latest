use axum::extract::Path;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::payment_handlers::{
    cancel_payment, delete_payment, get_merchant_payments, get_my_payments, make_payment,
    update_payment,
};
use crate::models::payment_models::{EditPayment, NewPayment};

pub fn payment_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>>  {
    let get_merchant_payments_pool = Arc::clone(&pool);
    let get_my_payments_pool = Arc::clone(&pool);
    let create_payment_pool = Arc::clone(&pool);
    let update_payment_pool = Arc::clone(&pool);
    let delete_payment_pool = Arc::clone(&pool);
    let cancel_payment_pool = Arc::clone(&pool);

    Router::new()
        .route(
            "/:merchant_id/payments",
            get(move |path: Path<i32>| get_merchant_payments(path, get_merchant_payments_pool)),
        )
        .route(
            "/my/:user_id/payments",
            get(move |path: Path<i32>| get_my_payments(path, get_my_payments_pool)),
        )
        .route(
            "/create",
            post(move |Json(new_payment): Json<NewPayment>| {
                make_payment(Json(new_payment), create_payment_pool)
            }),
        )
        .route(
            "/:payment_id/update",
            put(
                move |path: Path<i32>, Json(payment_data): Json<EditPayment>| {
                    update_payment(path, Json(payment_data), update_payment_pool)
                },
            ),
        )
        .route(
            "/:payment_id/delete",
            delete(move |path: Path<i32>| delete_payment(path, delete_payment_pool)),
        )
        .route(
            "/:payment_id/cancel",
            put(move |path: Path<i32>| cancel_payment(path, cancel_payment_pool)),
        )
}
