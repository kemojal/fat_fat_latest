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

pub fn register_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
   

    let user_pool = Arc::clone(&pool);
    let verify_user_pool = Arc::clone(&pool);


    // let user_service = UserServices::new(user_pool, verify_user_pool);
    Router::new()
        .route(
            "/register",
            post(move |Json(new_user): Json<NewUser>| {
                create_user(axum::Json(new_user), user_pool.clone())
            }),
        )
        .route(
            "/:email/verify",
            post(
                move |path: Path<String>, Json(verification_data): Json<VerifyUser>| {
                    verify_user(path, axum::Json(verification_data), verify_user_pool)
                },
            ),
        )
}
