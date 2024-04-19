use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;

use crate::handlers::transaction_handlers::{get_user_transactions, send_money};

use sqlx::PgPool;

pub fn transaction_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>> {
    Router::new()
        // .route("/", get( move || {get_user_transactions(get_pool)  }))
        .route("/:phone_number", get(get_user_transactions))
        .route("/:phone_number/send_money", post(send_money))
    // .route("/transaction/:email", get( move |path: Path<String>| {get_user_profile(path, get_user_pool)  }))
    // .route("/transaction/:number/create", post(move |path: Path<String>, Json(verification_data): Json<VerifyUser>| {
    //     verify_user(path, axum::Json(verification_data), verify_user_pool)
    // }))
    // .route("/transaction/:number/edit", put(move |path: Path<String>| {
    //     resend_verification_code(path, resendCodePool)
    // }))
    // .route("/transaction/delete/:id", delete(move |path: Path<i32>| {

    //     delete_user(path,  deleteUserPool)
    // }))
}
