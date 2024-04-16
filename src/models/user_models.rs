use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub verification_code: Option<String>,
    pub verified: Option<bool>,
    pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    pub updated_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
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
pub  struct EditUser {
    // pub  first_name: Option<String>,
    // pub  last_name: Option<String>,
    pub  email: Option<String>,
    // pub  password: Option<String>,
    // pub registration_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub  struct EditUserPassoword {
    // pub  first_name: Option<String>,
    // pub  last_name: Option<String>,
    pub  email: Option<String>,
    pub  password: String,
    // pub registration_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub  struct UserEmail {
    pub  email: Option<String>,
}

pub struct UserPhoneNumber{
    pub  phone_number: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub  struct SignUpUserEmail {
    pub  email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub  struct UserId {
    pub id: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyUser {
    pub verification_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToVerify {
    pub id: i32,
    pub email: Option<String>,
    pub verification_code: Option<String>,
    pub verified: Option<bool>,
    pub verification_code_created_at: Option<NaiveDateTime>, 
     
}


pub struct AuthorizedUser {
    pub id: i32,
    pub email: Option<String>,
    
}


#[derive(Debug, Serialize, Deserialize)]
pub  struct BalanceAmount {
    pub total_paid: i32
}


//unverified user registration process
#[derive(Debug, Deserialize)]
pub  struct UnverifiedUserPhoneNumber {
    pub  phone_number: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnverifiedUser {
    pub id: i32,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub user_name: Option<String>,
    pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    pub updated_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NewUnverifiedUser {
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub user_name: Option<String>,
    pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    pub updated_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}