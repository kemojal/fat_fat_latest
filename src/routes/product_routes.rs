
use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::product_handlers::{
    create_product, delete_product, get_merchant_products, get_qr_code, update_product,
};


pub fn product_routes(_pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
    
    Router::new()
        .route("/:merchant_id/products", get(get_merchant_products))
        .route("/:user_id/create", post(create_product))
        .route("/:product_id/qrcode", get(get_qr_code))
        .route("/:product_id/edit", put(update_product))
        .route("/:product_id/delete", delete(delete_product))
}
