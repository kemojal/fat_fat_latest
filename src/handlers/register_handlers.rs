use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use axum_macros::debug_handler;
use std::sync::Arc;
use tracing::error;

use crate::models::{
    user_models::{NewUnverifiedUser, UnverifiedUser, VerifyData},
    wallet_models::{Balance, GetUserBalanceError},
};
use crate::{
    middlewares::auth_middleware::decode_jwt_token,
    models::user_models::{
        EditUser, EditUserPassoword, NewUser, User, UserEmail, UserId, UserPhoneNumber,
        UserToVerify, VerifyUser,
    },
};
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Duration, Utc};
use lettre::message::{Mailbox, MultiPart};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::json;
use sqlx::{query, query_as, PgPool};

use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::Error as SmtpError;
use lettre::{Message, SmtpTransport, Transport};

use reqwest::Client;

use super::user_handlers::hash_password;

#[debug_handler]
pub async fn create_unverified_user(
    // pool: Arc<PgPool>,
    State(pool): State<Arc<PgPool>>,
    Json(register_data): Json<UserPhoneNumber>,
) -> impl IntoResponse {
    // let first_name = new_user.first_name;
    // let last_name = new_user.last_name;
    let phone_number = register_data.phone_number;

    //check if phone number already exis on unverfied
    let users_unverified_phone_numbers: Option<UserPhoneNumber> = query_as!(
        UserPhoneNumber,
        "SELECT phone_number FROM unverified_users WHERE phone_number = $1",
        phone_number
    )
    .fetch_optional(&*pool)
    .await
    .expect("Failed to fetch user");

    if users_unverified_phone_numbers.is_some() {
        // Phone number already exists
        return Json(json!({
            "status": "error",
            "message": "User with this phone number already exists in unverified_users"
        }));
    }

    //check if phone number already exist on users
    let users_phone_numbers: Option<UserPhoneNumber> = query_as!(
        UserPhoneNumber,
        "SELECT phone_number FROM users WHERE phone_number = $1",
        phone_number
    )
    .fetch_optional(&*pool)
    .await
    .expect("Failed to fetch user");

    if users_phone_numbers.is_some() {
        // Phone number already exists
        return Json(json!({
            "status": "error",
            "message": "User with this phone number already exists"
        }));
    }

    let phone_number_clone = phone_number.clone();

    // Send SMS
    let contact_number = match phone_number_clone {
        Some(number) => number,
        None => {
            return Json(json!({
                "status": "error",
                "message": "Phone number not provided"
            }));
        }
    };

    let sms_verification_code = generate_sms_verification_code();

    // Send SMS
    send_sms(&contact_number, &sms_verification_code)
        .await
        .unwrap_or_else(|e| {
            println!("Error sending SMS: {:?}", e);
        });

    // INSERT INTO users (first_name, last_name, email, password, registration_date)
    let query_result = query!(
        "
            INSERT INTO unverified_users (phone_number, verification_code )
            VALUES ($1, $2)
            RETURNING *",
        phone_number,
        sms_verification_code
    )
    .fetch_one(&*pool)
    .await;

    match query_result {
        Ok(row) => {
            let new_id = row.id;
            Json(json!({
                "status": "success",
                "message": "unverified user created successfully",
                "new_id": new_id
            }))
        }
        Err(_) => {
            // Handle error case
            // You can return an error response or customize it as needed
            // For now, let's return a generic error response
            Json(json!({
                "status": "error",
                "message": "Failed to create user"
            }))
        }
    }
}

#[debug_handler]
pub async fn verify_code(
    State(pool): State<Arc<PgPool>>,
    Json(data): Json<VerifyData>,
) -> impl IntoResponse {
    match query_as!(
        UnverifiedUser,
        "SELECT * FROM unverified_users WHERE phone_number = $1 AND verification_code = $2",
        data.phone_number,
        data.verification_code
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(user) => {
            let now = Utc::now();
            let created_at = user.created_at.unwrap();
            let created_at_utc = DateTime::<Utc>::from_utc(created_at, Utc);
            let diff = now - created_at_utc;

            // print!("Diff: {:?}", diff);
            if diff > Duration::minutes(30) {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Verification code expired" })),
                );
            }

            if user.phone_verified.unwrap_or(false) {
            } else {
                sqlx::query!(
                    "UPDATE unverified_users SET phone_verified = true WHERE id = $1",
                    user.id
                )
                .execute(pool.as_ref())
                .await
                .unwrap();
            }

            (
                StatusCode::OK,
                Json(json!({ "message": "Phone number verified successfully" })),
            )
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Failed to verify user: {}", e) })),
        ),
    }
}



pub async fn complete_registration(
    State(pool): State<Arc<PgPool>>,
    Json(data): Json<NewUnverifiedUser>,
) -> impl IntoResponse {
    let user = match sqlx::query_as!(
        UnverifiedUser,
        "SELECT * FROM unverified_users WHERE phone_number = $1",
        data.phone_number
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Failed to fetch user: {}", e) })),
            );
        }
    };

    if !user.phone_verified.unwrap_or(false) {

        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Phone number not verified" })),
        );
    }

    if sqlx::query!(
        "SELECT username FROM users WHERE username = $1",
        data.username
    )
    .fetch_optional(pool.as_ref())
    .await
    .unwrap()
    .is_some()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Username already registered" })),
        );
    }

    if sqlx::query!(
        "SELECT email FROM users WHERE email = $1",
        data.email
    )
    .fetch_optional(pool.as_ref())
    .await
    .unwrap()
    .is_some()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Email already registered" })),
        );
    }

    // let password_hash = hash(data.password, DEFAULT_COST).unwrap();

    let password = data.password.clone();

    let hashed_password = hash_password(password.as_ref().unwrap());

    // match sqlx::query!(
    //     "UPDATE unverified_users
    //     SET username = $1, email = $2, password_hash = $3 
    //     WHERE phone_number = $4",
    //     data.username,
    //     data.email,
    //     hashed_password,
    //     data.phone_number
    // )
    match sqlx::query!(
        "INSERT INTO users (username, email, password_hash, phone_number )
        VALUES ($1, $2, $3, $4)
        RETURNING *",
        data.username,
        data.email,
        hashed_password,
        data.phone_number
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(_) => {
            //delete this person with this phone on unverfied_users table
            let delete_result =  query!(
                "DELETE FROM unverified_users WHERE phone_number = $1",
                data.phone_number
            ).execute(&*pool)
            .await;

            if delete_result.is_ok() {
                return (
                    StatusCode::OK,
                    Json(json!({ "message": "Registration complete" })),
                )
            } else {
                // Return an error response
                return (
                    StatusCode::OK,
                    Json(json!({ "message": "Registration not complete" })),
                )
            }

            
        },
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Failed to complete registration: {}", e) })),
        ),
    }
}



async fn send_sms(
    phone_number: &str,
    verification_code: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let account_sid = "ACe0278dc21b695259a2d831d2a887fae5";
    let auth_token = "c9c388dd54160f89256e7e8b87b0d3aa";
    let service_sid = "VA61f2dfef6c1ed74eaffa5ff8a2aca098";
    let from_phone_number = "+14696208723";

    let body = format!(
        "Confirm your phone number on Nafa with the code: { }. Don't share this code with anyone.",
        verification_code
    );

    let client = Client::new();
    let response = client
        .post(&format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            account_sid
        ))
        .basic_auth(account_sid, Some(auth_token))
        .form(&[
            ("To", phone_number),
            ("From", from_phone_number),
            ("Body", &body),
        ])
        .send()
        .await?;

    if response.status().is_success() {
        println!("SMS sent successfully!");
    } else {
        println!("Failed to send SMS: {:?}", response.text().await?);
    }

    Ok(())
}

fn generate_sms_verification_code() -> String {
    let length = 6;
    let mut rng = thread_rng();

    let code: String = (0..length)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect();

    code
}

fn generate_verification_code() -> String {
    let length = 6;
    let rng = thread_rng();

    let code: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    code
}
