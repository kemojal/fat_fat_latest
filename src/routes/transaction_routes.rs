use std::sync::Arc;
use axum::extract::Path;
use axum::{Json, Router};
use axum::routing::{delete, get, post, put};

use sqlx::PgPool;
use crate::handlers::transaction_handlers::{get_user_transactions, send_money};
use crate::models::transaction_models::NewTransaction;






pub fn transaction_routes(pool: Arc<PgPool>) -> Router<Arc<PgPool>>  {
    let get_pool = Arc::clone(&pool);
    let get_user_pool = Arc::clone(&pool);


    let user_pool = Arc::clone(&pool);
    let verify_user_pool = Arc::clone(&pool);
    


    Router::new()
        // .route("/", get( move || {get_user_transactions(get_pool)  }))
        .route("/:phone_number", get( move |path: Path<String>| {get_user_transactions(path,get_pool)  }))
        .route("/:phone_number/send_money", post(move |path: Path<String>, Json(new_transaction): Json<NewTransaction>| {
            send_money(path, axum::Json(new_transaction),user_pool.clone())
        }))
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