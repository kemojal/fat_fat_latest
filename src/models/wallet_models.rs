

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;



#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub id: i32,
    pub user_id: Option<i32>,
    pub balance: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
    pub created_at: Option<NaiveDateTime>, 
    pub updated_at:Option<NaiveDateTime>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewWallet {
    pub user_id: Option<i32>,
    pub balance: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub balance: Option<sqlx::types::BigDecimal>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DepositAmount {
    pub amount: Option<sqlx::types::BigDecimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawAmount {
    pub amount: Option<sqlx::types::BigDecimal>,
}


#[derive(Debug, Error)]
pub enum GetUserBalanceError {
    #[error("User not found")]
    UserNotFound,
    #[error("No wallet found for this user")]
    WalletNotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] anyhow::Error),
}





