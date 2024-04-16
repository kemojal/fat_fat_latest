use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub email_host: String,
    pub email_username: String,
    pub email_password: String,
    pub email_sender_name: String,
    pub email_sender_address: String,
    pub twilio_account_sid: String,
    pub twilio_auth_token: String,
    pub twilio_service_sid: String,
    pub twilio_from_phone_number: String,
}

impl AppConfig {
    pub fn load() -> Self {
        
        let config = envy::from_env::<AppConfig>().expect("Failed to load config");
        config
    }
}