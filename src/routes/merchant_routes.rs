use axum::extract::Path;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::merchant_handlers::{
    create_merchant, delete_merchant, edit_merchant, get_merchant,
};

use crate::models::merchant_models::{EditMerchant, NewMerchant};


pub fn merchant_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>>  {
    let get_my_merchant_pool = Arc::clone(&pool);

    let edit_pool = Arc::clone(&pool);
    let create_pool = Arc::clone(&pool);
    let delete_pool = Arc::clone(&pool);
    let delete_merchant_pool = Arc::clone(&pool);

    Router::new()
        // .route("/", get( move || {get_issues(get_pool)  }))
        // .route("/:user_id", get( move |path: Path<i32>,| {
        //     get_issues_by_workspace_id(path, get_user_pool)
        // }))
        .route(
            "/:username/merchants",
            get(move |path: Path<String>| get_merchant(path, get_my_merchant_pool)),
        )
        // .route("/:username/my_issues/created", get( move |path: Path<String>| {
        //     get_my_issues_created(path, get_my_issues_created_pool)
        // }))
        .route(
            "/:username/create",
            post(
                move |path: Path<String>, Json(new_merchant): Json<NewMerchant>| {
                    create_merchant(path, Json(new_merchant), create_pool)
                },
            ),
        )
        .route(
            "/:merchant_id/edit",
            put(
                move |path: Path<i32>, Json(payload): Json<EditMerchant>| {
                    edit_merchant(path, Json(payload), edit_pool)
                },
            ),
        )
        .route(
            "/:product_id/delete",
            delete(move |path: Path<i32>| delete_merchant(path, delete_merchant_pool)),
        )
}
