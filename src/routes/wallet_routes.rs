use axum::extract::Path;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use sqlx::PgPool;
use std::sync::Arc;
// use crate::handlers::issue_handlers::{create_issue, get_issues, get_issues_by_workspace_id, get_my_issues_all, get_my_issues_created, get_issues_by_workspace_slug};
use crate::handlers::wallet_handlers::{
    create_wallet, delete_wallet, get_wallets, get_wallets_by_user_id,
};
use crate::models::wallet_models::NewWallet;
// use crate::handlers::workspace_handlers::create_workspace;

pub fn wallet_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
    Router::new()
        .route("/", get(get_wallets))
        .route("/:user_id", get(get_wallets_by_user_id))
        .route("/:username/create", post(create_wallet))
        .route("/:wallet_id/delete", delete(delete_wallet))
}
