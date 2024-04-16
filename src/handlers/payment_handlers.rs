use crate::models::merchant_models::MerchantUserId;
use crate::models::payment_models::{EditPayment, NewPayment, Payment};


use axum::extract::Path;

use axum::response::{IntoResponse, Json};



use reqwest::StatusCode;
use serde_json::json;
use sqlx::{query, query_as, PgPool};
use std::sync::Arc;








pub async fn make_payment(
    Json(new_payment): Json<NewPayment>,
    pool: Arc<PgPool>,
) -> impl IntoResponse {
    // let user_wallet_result: Vec<Wallet> = query_as!(
    //     Wallet,
    //     "
    //     SELECT id, user_id, balance, currency, created_at, updated_at
    //     FROM wallets
    //     WHERE user_id = $1
    //     ",
    //     new_payment.user_id
    // )
    // .fetch_all(&*pool)
    // .await
    // .expect("Failed to fetch user account information");

    // if let Some(first_user_wallet) = user_wallet_result.get(0) {
    //     if first_user_wallet.balance >= new_payment.amount
    //         && first_user_wallet.currency == new_payment.currency
    //     {
    //         // let updated_user_balance = match (first_user_wallet.balance.clone(), new_payment.amount.clone()){
    //         //     (Some(balance), Some(amt)) => {
    //         //         let new_balance = balance - amt;
    //         //         Some(new_balance)
    //         //     }
    //         //     _ => None,

    //         // }
    //     } else {
    //         println!("not enough funds or same currency")
    //     }
    // }

    let merchant_user_id: Option<MerchantUserId> = query_as!(
        MerchantUserId,
        "SELECT user_id FROM merchants WHERE id = $1",
        new_payment.merchant_id
    )
    .fetch_optional(&*pool)
    .await
    .expect("Failed to fetch merchant's user ID");

    match merchant_user_id {
        Some(merchant_user_id) => {
            let mut transaction = pool.begin().await.expect("Failed to start transaction");

            // Add the paid amount to the merchant's wallet balance
            let merchant_wallet_update_result = query!(
                "UPDATE wallets SET balance = balance + $1 WHERE user_id = $2",
                new_payment.amount,
                merchant_user_id.user_id.clone(),
            )
            .execute(&mut *transaction)
            .await;

            if let Err(e) = merchant_wallet_update_result {
                transaction
                    .rollback()
                    .await
                    .expect("Failed to rollback transaction");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to update merchant's wallet balance: {}", e)
                    })),
                )
                    .into_response();
            }

            // Subtract the paid amount from the user's wallet balance
            let user_wallet_update_result = query!(
                "UPDATE wallets SET balance = balance - $1 WHERE user_id = $2",
                new_payment.amount,
                new_payment.user_id
            )
            .execute(&mut *transaction)
            .await;

            if let Err(e) = user_wallet_update_result {
                transaction
                    .rollback()
                    .await
                    .expect("Failed to rollback transaction");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to update user's wallet balance: {}", e)
                    })),
                )
                    .into_response();
            }

            let result = query!(
                "INSERT INTO payments (merchant_id, user_id, amount, currency, product_id, status)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id",
                new_payment.merchant_id,
                new_payment.user_id,
                new_payment.amount,
                new_payment.currency,
                new_payment.product_id,
                new_payment.status
            )
            .fetch_one(&mut *transaction)
            .await;

            match result {
                Ok(row) => {
                    transaction
                        .commit()
                        .await
                        .expect("Failed to commit transaction");
                    return (
                        StatusCode::OK,
                        Json(json!({
                            "status": "success",
                            "message": "Payment made successfully",
                            "new_payment_id": row.id
                        })),
                    )
                        .into_response();
                }
                Err(e) => {
                    transaction
                        .rollback()
                        .await
                        .expect("Failed to rollback transaction");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "status": "error",
                            "message": format!("Failed to make payment: {}", e)
                        })),
                    )
                        .into_response();
                }
            }
        }
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "This merchant is not connected to any user"
                })),
            )
                .into_response();
        }
    }
}

pub async fn get_merchant_payments(
    Path(merchant_id): Path<i32>,
    pool: Arc<PgPool>,
) -> impl IntoResponse {
    let payments: Vec<Payment> = query_as!(
        Payment,
        "
        SELECT * FROM payments WHERE merchant_id = $1
        ",
        merchant_id
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch payments");

    Json(payments)
}

pub async fn get_my_payments(Path(user_id): Path<i32>, pool: Arc<PgPool>) -> impl IntoResponse {
    let payments: Vec<Payment> = query_as!(
        Payment,
        "
        SELECT * FROM payments WHERE user_id = $1
        ",
        user_id
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch payments");

    Json(payments)
}

pub async fn update_payment(
    Path(payment_id): Path<i32>,
    Json(payment_data): Json<EditPayment>,
    pool: Arc<PgPool>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "UPDATE payments SET amount = $1, currency = $2, product_id = $3, status = $4 WHERE id = $5",
    )
    .bind(payment_data.amount)
    .bind(payment_data.currency)
    .bind(payment_data.product_id)
    .bind(payment_data.status)
    .bind(payment_id)
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json("Payment details updated")).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed to update payment details: {}", e)),
        )
            .into_response(),
    }
}

pub async fn delete_payment(Path(payment_id): Path<i32>, pool: Arc<PgPool>) -> impl IntoResponse {
    let result = query!(
        "
        DELETE FROM payments
        WHERE id = $1
        RETURNING id
        ",
        payment_id
    )
    .fetch_one(&*pool)
    .await;

    match result {
        Ok(row) => {
            let deleted_id = row.id;
            Json(json!({
                "status": "success",
                "message": "Payment deleted successfully",
                "deleted_id": deleted_id
            }))
        }
        Err(e) => {
            println!("Error deleting payment: {:?}", e);
            Json(json!({
                "status": "error",
                "message": format!("Failed to delete payment: {:?}", e)
            }))
        }
    }
}

pub async fn cancel_payment(Path(payment_id): Path<i32>, pool: Arc<PgPool>) -> impl IntoResponse {
    let result = sqlx::query("UPDATE payments SET status = 'cancelled' WHERE id = $1")
        .bind(payment_id)
        .execute(&*pool)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, Json("Payment cancelled successfully")).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed to cancel payment: {}", e)),
        )
            .into_response(),
    }
}
