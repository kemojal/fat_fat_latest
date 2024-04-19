
use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;

use crate::handlers::user_handlers::{
    create_user, delete_user, edit_user, get_user_balance, get_user_profile, get_users,
    resend_verification_code, verify_user,
};

use sqlx::PgPool;

// use crate::services::UserServices;

pub fn user_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
    
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
