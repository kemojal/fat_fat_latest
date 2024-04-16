use std::sync::Arc;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Duration, Utc};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest::Client;
use serde_json::json;
use sqlx::{query, query_as, PgPool};

use crate::config::AppConfig;
use crate::models::user_models::{
    EditUser, EditUserPassoword, NewUser, User, UserEmail, UserId, UserPhoneNumber, UserToVerify,
    VerifyUser,
};
use crate::utils::sms_utils::send_sms;

pub struct UserService {
    pub pool: Arc<PgPool>,
    pub config: Arc<AppConfig>,
}

impl UserService {
    pub async fn get_users(&self) -> impl IntoResponse {
        // Implementation
    }

    pub async fn get_user_profile(&self, email: &str) -> impl IntoResponse {
        // Implementation
    }

    pub async fn get_user_balance(&self, email: &str) -> impl IntoResponse {
        // Implementation
    }

    pub async fn create_user(&self, new_user: NewUser) -> impl IntoResponse {
        // Implementation
    }

    pub async fn verify_user(&self, email: &str, verification_data: VerifyUser) -> impl IntoResponse {
        // Implementation
    }

    pub async fn resend_verification_code(&self, email: &str) -> impl IntoResponse {
        // Implementation
    }

    pub async fn edit_user(&self, id: i32, edit_user_data: EditUser) -> impl IntoResponse {
        // Implementation
    }

    pub async fn edit_user_password(&self, id: i32, edit_user_data: EditUserPassoword) -> impl IntoResponse {
        // Implementation
    }

    pub async fn delete_user(&self, id: i32) -> impl IntoResponse {
        // Implementation
    }

    fn hash_password(&self, password: &str) -> String {
        // Implementation
    }

    fn generate_verification_code(&self) -> String {
        // Implementation
    }

    async fn send_verification_email(&self, email_address: &str, verification_code: &str) -> Result<(), lettre::transport::smtp::Error> {
        // Implementation
    }
}