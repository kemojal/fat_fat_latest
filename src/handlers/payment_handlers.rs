use crate::models::merchant_models::MerchantUserId;
use crate::models::payment_models::{EditPayment, NewPayment, Payment};
use crate::services::payment_service::get_merchant_user_id;

use axum::extract::{Path, State};

use axum::response::{IntoResponse, Json};

use reqwest::StatusCode;
use serde_json::json;
// use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use std::sync::Arc;

use super::auth_handlers::AuthError;

// pub async fn make_payment2(
//     State(pool): State<Arc<PgPool>>,
//     Json(new_payment): Json<NewPayment>,
// ) -> impl IntoResponse {
//     let merchant_user_id = get_merchant_user_id(&pool, new_payment.merchant_id)
//         .await
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

//     let mut transaction = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e));

//     update_merchant_wallet(&mut transaction, merchant_user_id.user_id, new_payment.amount)
//         .await
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

//     update_user_wallet(&mut transaction, new_payment.user_id, -new_payment.amount)
//         .await
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

//     let payment_id = insert_payment(&mut transaction, &new_payment)
//         .await
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

//     transaction.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

//     (StatusCode::OK, Json(json!({
//         "status": "success",
//         "message": "Payment made successfully",
//         "new_payment_id": payment_id
//     }))).into_response()
// }

pub async fn make_payment(
    // pool: Arc<PgPool>,
    State(pool): State<Arc<PgPool>>,
    Json(new_payment): Json<NewPayment>,
) -> Result<impl IntoResponse, AuthError> {
    let merchant_user_id: Option<MerchantUserId> = query_as!(
        MerchantUserId,
        "SELECT user_id FROM merchants WHERE id = $1",
        new_payment.merchant_id
    )
    .fetch_optional(&*pool)
    .await
    .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
    // .expect("Failed to fetch merchant's user ID");

    match merchant_user_id {
        Some(merchant_user_id) => {
            let mut transaction = pool
                .begin()
                .await
                .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            // .expect("Failed to start transaction");

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
                    .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
                // .expect("Failed to rollback transaction");
                return Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to update merchant's wallet balance: {}", e)
                    })),
                )
                    .into_response());
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
                    .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
                // .expect("Failed to rollback transaction");
                return Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to update user's wallet balance: {}", e)
                    })),
                )
                    .into_response());
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
                        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
                    // .expect("Failed to commit transaction");
                    return Ok((
                        StatusCode::OK,
                        Json(json!({
                            "status": "success",
                            "message": "Payment made successfully",
                            "new_payment_id": row.id
                        })),
                    )
                        .into_response());
                }
                Err(e) => {
                    transaction
                        .rollback()
                        .await
                        .expect("Failed to rollback transaction");
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "status": "error",
                            "message": format!("Failed to make payment: {}", e)
                        })),
                    )
                        .into_response());
                }
            }
        }
        None => {
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "This merchant is not connected to any user"
                })),
            )
                .into_response());
        }
    }
}

pub async fn get_merchant_payments(
    Path(merchant_id): Path<i32>,
    // pool: Arc<PgPool>,
    State(pool): State<Arc<PgPool>>,
) -> Result<impl IntoResponse, AuthError> {
    let payments: Vec<Payment> = query_as!(
        Payment,
        "
        SELECT * FROM payments WHERE merchant_id = $1
        ",
        merchant_id
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
    // .expect("Failed to fetch payments");

    Ok(Json(payments))
}

pub async fn get_my_payments(
    Path(user_id): Path<i32>,
    // pool: Arc<PgPool>
    State(pool): State<Arc<PgPool>>,
) -> Result<impl IntoResponse, AuthError> {
    let payments: Vec<Payment> = query_as!(
        Payment,
        "
        SELECT * FROM payments WHERE user_id = $1
        ",
        user_id
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
    // .expect("Failed to fetch payments");

    Ok(Json(payments))
}

pub async fn update_payment(
    Path(payment_id): Path<i32>,
    // pool: Arc<PgPool>,
    State(pool): State<Arc<PgPool>>,
    Json(payment_data): Json<EditPayment>,
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

pub async fn delete_payment(
    Path(payment_id): Path<i32>,
    // pool: Arc<PgPool>
    State(pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
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

pub async fn cancel_payment(
    Path(payment_id): Path<i32>,
    // pool: Arc<PgPool>
    State(pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
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
