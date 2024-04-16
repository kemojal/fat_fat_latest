use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: i32,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub verification_code: Option<String>,
    pub verified: Option<bool>,
    pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    pub updated_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAgent {
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub phone_number: Option<String>,
    // pub verification_code: Option<String>,
    // pub verified: Option<bool>,
    // pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    // pub updated_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}

#[derive(Deserialize)]
pub struct EditAgent {
    // pub  first_name: Option<String>,
    // pub  last_name: Option<String>,
    pub email: Option<String>,
    // pub  password: Option<String>,
    // pub registration_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct EditAgentPassoword {
    // pub  first_name: Option<String>,
    // pub  last_name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    // pub registration_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct AgentEmail {
    pub email: Option<String>,
}

pub struct AgentPhoneNumber {
    pub phone_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpAgentEmail {
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentId {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyAgent {
    pub verification_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentToVerify {
    pub id: i32,
    pub email: Option<String>,
    pub verification_code: Option<String>,
    pub verified: Option<bool>,
    pub verification_code_created_at: Option<NaiveDateTime>,
}

pub struct AuthorizedAgent {
    pub id: i32,
    pub email: Option<String>,
}
