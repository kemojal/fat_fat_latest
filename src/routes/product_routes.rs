use axum::extract::Path;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::product_handlers::{
    create_product, delete_product, get_merchant_products, get_qr_code, update_product,
};
use crate::models::product_models::{EditProduct, NewProduct};

pub fn product_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>>  {
    let get_merchant_product_pool = Arc::clone(&pool);

    let edit_pool = Arc::clone(&pool);
    let create_pool = Arc::clone(&pool);
    let create_qrcode_pool = Arc::clone(&pool);

    let delete_merchant_pool = Arc::clone(&pool);

    Router::new()
        .route(
            "/:merchant_id/products",
            get(move |path: Path<i32>| get_merchant_products(path, get_merchant_product_pool)),
        )
        .route(
            "/:user_id/create",
            post(
                move |path: Path<i32>, Json(new_product): Json<NewProduct>| {
                    create_product(path, Json(new_product), create_pool)
                },
            ),
        )
        .route(
            "/:product_id/qrcode",
            get(
                move |path: Path<i32>, Json(new_product): Json<NewProduct>| {
                    get_qr_code(path, create_qrcode_pool)
                },
            ),
        )
        .route(
            "/:product_id/edit",
            put(
                move |path: Path<i32>, Json(product_data): Json<EditProduct>| {
                    update_product(path, Json(product_data), edit_pool)
                },
            ),
        )
        .route(
            "/:product_id/delete",
            delete(move |path: Path<i32>| delete_product(path, delete_merchant_pool)),
        )
}
