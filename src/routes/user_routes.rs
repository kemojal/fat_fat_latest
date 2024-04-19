use axum::extract::Path;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use std::sync::Arc;

use crate::handlers::user_handlers::{
    create_user, delete_user, edit_user, get_user_balance, get_user_profile, get_users,
    resend_verification_code, verify_user,
};
use crate::models::user_models::{EditUser, NewUser, VerifyUser};
use sqlx::{PgPool, Pool, Postgres};

// use crate::services::UserServices;

pub fn user_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
    // let get_pool = Arc::clone(&pool);
    // let get_user_pool = Arc::clone(&pool);

    let user_pool = Arc::clone(&pool);
    let verify_user_pool = Arc::clone(&pool);
    // let get_user_pool = Arc::clone(&pool);
    // let get_user_balance_pool = Arc::clone(&pool);
    // let resend_code_pool = Arc::clone(&pool);
    let edit_user_pool = Arc::clone(&pool);
    let edit_user_password_pool = Arc::clone(&pool);
    // let delete_user_pool = Arc::clone(&pool);

    // let user_service = UserServices::new(user_pool, verify_user_pool);
    Router::new()
        .route("/users", get(get_users))
        .route("/create", post(create_user))
        .route("/:email/profile", get(get_user_profile))
        .route("/:email/balance", get(get_user_balance))
        .route("/:email/verify", post(verify_user))
        .route(
            "/:email/resend_verification_code",
            put(resend_verification_code),
        )
        .route("/edit/:id", put(edit_user))
        .route("/edit/:id/password", put(edit_user))
        .route("/delete/:id", delete(delete_user))
}
