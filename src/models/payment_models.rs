use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub id: i32,
    pub merchant_id: i32,
    pub user_id: i32,
    pub amount: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
    pub payment_date: Option<NaiveDateTime>,
    pub product_id: Option<i32>,
    pub status: Option<String>,
    pub edited_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditPayment {
    pub merchant_id: i32,
    pub user_id: i32,
    pub amount: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
    pub product_id: Option<i32>,
    pub status: Option<String>,
    // pub edited_at: Option<NaiveDateTime>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPayment {
    pub merchant_id: i32,
    pub user_id: i32,
    pub amount: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
    pub product_id: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentId {
    pub id: i32,
}
