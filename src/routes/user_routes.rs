use axum::extract::Path;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use std::sync::Arc;

use crate::handlers::user_handlers::{
    create_user, delete_user, edit_user, 
    get_user_balance, get_user_profile, 
     get_users, 
    resend_verification_code, verify_user
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
        .route(
            "/create",
            post(move |Json(new_user): Json<NewUser>| {
                create_user(axum::Json(new_user), user_pool.clone())
            }),
        )
        .route(
            "/:email/profile",
            get( get_user_profile),
        )
        .route(
            "/:email/balance",
            get(get_user_balance),
        )
        .route(
            "/:email/verify",
            post(
                move |path: Path<String>, Json(verification_data): Json<VerifyUser>| {
                    verify_user(path, axum::Json(verification_data), verify_user_pool)
                },
            ),
        )
        .route(
            "/:email/resend_verification_code",
            put(resend_verification_code),
        )
        .route(
            "/edit/:id",
            put(
                move |path: Path<i32>, Json(edit_user_data): Json<EditUser>| {
                    edit_user(path, Json(edit_user_data), edit_user_pool)
                },
            ),
        )
        .route(
            "/edit/:id/password",
            put(
                move |path: Path<i32>, Json(edit_user_data): Json<EditUser>| {
                    edit_user(path, Json(edit_user_data), edit_user_password_pool)
                },
            ),
        )
        .route(
            "/delete/:id",
            delete(delete_user),
        )
}
