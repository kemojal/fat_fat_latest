use std::sync::Arc;
use axum::extract::Path;
use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use sqlx::PgPool;
// use crate::handlers::issue_handlers::{create_issue, get_issues, get_issues_by_workspace_id, get_my_issues_all, get_my_issues_created, get_issues_by_workspace_slug};
use crate::handlers::wallet_handlers::{create_wallet, delete_wallet, get_wallets, get_wallets_by_user_id };
use crate::models::wallet_models::NewWallet;
// use crate::handlers::workspace_handlers::create_workspace;




pub fn wallet_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>>  {
    let get_pool = Arc::clone(&pool);
    let deposit_pool = Arc::clone(&pool);
    let withdraw_pool = Arc::clone(&pool);
    let get_slug_pool = Arc::clone(&pool);
    let get_user_pool = Arc::clone(&pool);
    let get_my_issues_pool = Arc::clone(&pool);
    let create_pool = Arc::clone(&pool);
    let get_my_issues_created_pool = Arc::clone(&pool);
    let delete_pool = Arc::clone(&pool);
    let get_issues_by_workspace_id_pool = Arc::clone(&pool);




    Router::new()
        .route("/", get( move || {get_wallets(get_pool)  }))
        .route("/:user_id", get( move |path: Path<i32>,| {
            get_wallets_by_user_id(path, get_user_pool)
        }))
        // .route("/edit/:id", put(move |path: Path<i32>, Json(deposit_data): Json<DepositAmount>| {
        //     deposit(path, Json(deposit_data), deposit_pool)
        // }))
        
    //     .route("/:user_id/deposit", put(  move|path: Path<i32>, Json(deposit_data): Json<DepositAmount>| {
    //         deposit(
    //             path,
    //             Json(deposit_data),
    //            deposit_pool
    //         )
    //     }
    // ))
        // .route("/:user_id/withdraw", post( move |path: Path<i32>,| {
        //     withdraw(path, withdraw_pool)
        // }))
        
        .route("/:username/create", post(move | path: Path<String>, Json(new_wallet): Json<NewWallet>| {
            create_wallet(path, Json(new_wallet), create_pool)
        }))
        .route("/:wallet_id/delete", delete(move |path: Path<i32>| {
            delete_wallet(path, delete_pool)
        }))
}