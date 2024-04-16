use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub merchant_id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<sqlx::types::BigDecimal>,
    pub is_product: bool,
    pub is_discounted: bool,
    pub discounted_amount: Option<sqlx::types::BigDecimal>,
    pub on_sale: bool,
    pub on_sale_amount: Option<sqlx::types::BigDecimal>,
    pub created_at: Option<NaiveDateTime>,
    pub edited_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditProduct {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<sqlx::types::BigDecimal>,
    pub is_product: bool,
    pub is_discounted: bool,
    pub discounted_amount: Option<sqlx::types::BigDecimal>,
    pub on_sale: bool,
    pub on_sale_amount: Option<sqlx::types::BigDecimal>,
    // pub edited_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<sqlx::types::BigDecimal>,
    pub is_product: bool,
    pub is_discounted: bool,
    pub discounted_amount: Option<sqlx::types::BigDecimal>,
    pub on_sale: bool,
    pub on_sale_amount: Option<sqlx::types::BigDecimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductId {
    pub id: i32,
}
