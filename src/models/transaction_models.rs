use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub sender_id: Option<i32>,
    pub recipient_id: Option<i32>,
    pub amount: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub transaction_type: Option<String>,
    pub transaction_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTransaction {
    pub sender_id: Option<i32>,
    pub recipient_id: Option<i32>,
    pub amount: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub transaction_type: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct TransactionWithUserDetails {
    pub id: i32,
    pub sender_id: Option<i32>,
    pub recipient_id: Option<i32>,
    pub amount: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub transaction_type: Option<String>,
    pub transaction_date: Option<NaiveDateTime>,
    pub sender_username: Option<String>,
    pub sender_phone_number: Option<String>,
    pub sender_email: Option<String>,
    pub recipient_username: Option<String>,
    pub recipient_phone_number: Option<String>,
    pub recipient_email: Option<String>,
}

// #[derive(Debug, Serialize)]
// pub struct TransactionUser {
//     pub username: String,
//     pub phone_number: String,
//     pub email: String,
// }

// #[derive(Debug, Serialize)]
// pub struct TransactionWithUsers {
//     pub id: i32,
//     pub sender: TransactionUser,
//     pub recipient: TransactionUser,
//     pub amount: Option<sqlx::types::BigDecimal>,
//     pub currency: Option<String>,
//     pub status: Option<String>,
//     pub transaction_type: Option<String>,
//     pub transaction_date: Option<NaiveDateTime>,
// }

// pub struct DepositAmount {
//     pub amount: Option<sqlx::types::BigDecimal>,
// }

// pub struct WithdrawAmount {
//     pub amount: Option<sqlx::types::BigDecimal>,
// }
