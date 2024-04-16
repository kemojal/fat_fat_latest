
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Merchant {
    pub id: i32,
    pub description: Option<String>,
    pub latitude: Option<sqlx::types::BigDecimal>,
    pub longitude: Option<sqlx::types::BigDecimal>,
    pub created_at: Option<NaiveDateTime>,
    pub business_name: Option<String>,
    pub business_type: Option<String>,
    pub address: Option<String>,
    pub business_phone_number: Option<String>,
    pub website: Option<String>,
    pub edited_at: Option<NaiveDateTime>,
    pub user_id: Option<i32>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EditMerchant {
   
    pub description: Option<String>,
    pub business_name: Option<String>,
    pub business_type: Option<String>,
    pub address: Option<String>,
    pub business_phone_number: Option<String>,
    pub website: Option<String>,
    pub edited_at: Option<NaiveDateTime>
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMerchant {
    pub description: Option<String>,
    // pub latitude: f64,
    // pub longitude: f64,
    // pub created_at: Option<NaiveDateTime>,
    pub business_name: Option<String>,
    pub business_type: Option<String>,
    pub address: Option<String>,
    pub business_phone_number: Option<String>,
    pub website: Option<String>,
    // pub edited_at: Option<NaiveDateTime>,
    // pub user_id: i32,
}


#[derive(Deserialize)]
pub struct EditMerchantPassoword {
    // pub  first_name: Option<String>,
    // pub  last_name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    // pub registration_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct MerchantEmail {
    pub email: Option<String>,
}

pub struct MerchantPhoneNumber {
    pub phone_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpMerchantEmail {
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantId {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantUserId {
    pub user_id: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyMerchant {
    pub verification_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantToVerify {
    pub id: i32,
    pub email: Option<String>,
    pub verification_code: Option<String>,
    pub verified: Option<bool>,
    pub verification_code_created_at: Option<NaiveDateTime>,
}

pub struct AuthorizedMerchant {
    pub id: i32,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceAmount {
    pub total_paid: i32,
}
