


// services and helper query methods

use axum::Json;
use reqwest::StatusCode;
use serde_json::json;
use sqlx::{postgres::PgQueryResult, query, query_as, PgConnection, PgPool};

use crate::models::{merchant_models::MerchantUserId, payment_models::NewPayment};

pub async fn get_merchant_user_id(pool: &PgPool, merchant_id: i32) -> Result<MerchantUserId, (StatusCode, Json<serde_json::Value>)> {
    query_as!(
        MerchantUserId,
        "SELECT user_id FROM merchants WHERE id = $1",
        merchant_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
        "status": "error",
        "message": format!("Failed to fetch merchant's user ID: {}", e)
    }))))?
    .ok_or((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
        "status": "error",
        "message": "This merchant is not connected to any user"
    }))))
}


pub async fn update_merchant_wallet(
    transaction: &mut PgConnection,
    user_id: i32,
    amount: i64,
) -> Result<PgQueryResult, (StatusCode, Json<serde_json::Value>)> {
    let amount_in_bigdecimal  = Some(Into::<sqlx::types::BigDecimal>::into(amount));
    query!(
        "UPDATE wallets SET balance = balance + $1 WHERE user_id = $2",
        amount_in_bigdecimal,
        user_id
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Failed to update merchant's wallet balance: {}", e)
            })),
        )
    })
}


pub async fn update_user_wallet(transaction: &mut PgConnection, user_id: i32, amount: i64) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let amount_in_bigdecimal  = Some(Into::<sqlx::types::BigDecimal>::into(amount));
    query!(
        "UPDATE wallets SET balance = balance - $1 WHERE user_id = $2",
        amount_in_bigdecimal,
        user_id
    )
    .execute(transaction)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
        "status": "error",
        "message": format!("Failed to update user's wallet balance: {}", e)
    }))))?;

    Ok(())
}

pub async fn insert_payment(transaction: &mut PgConnection, payment: &NewPayment) -> Result<i32, (StatusCode, Json<serde_json::Value>)> {
    query!(
        "INSERT INTO payments (merchant_id, user_id, amount, currency, product_id, status)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id",
        payment.merchant_id,
        payment.user_id,
        payment.amount,
        payment.currency,
        payment.product_id,
        payment.status
    )
    .fetch_one(transaction)
    .await
    .map(|row| row.id)
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
        "status": "error",
        "message": format!("Failed to make payment: {}", e)
    }))))
}