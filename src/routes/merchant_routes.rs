use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::merchant_handlers::{
    create_merchant, delete_merchant, edit_merchant, get_merchant,
};

pub fn merchant_routes(_pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
    Router::new()
        .route("/:username/merchants", get(get_merchant))
        .route("/:username/create", post(create_merchant))
        .route("/:merchant_id/edit", put(edit_merchant))
        .route("/:product_id/delete", delete(delete_merchant))
}
