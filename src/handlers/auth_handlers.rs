use crate::models::auth_models::{AuthUser, Claims, SignInData};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::{query_as, PgPool};
use std::{env, sync::Arc};

pub async fn sign_in(
    State(pool): State<Arc<PgPool>>,
    Json(signin_data): Json<SignInData>,
) -> Result<impl IntoResponse, AuthError> {
    // Extract username and password from signin_data
    let user_email = signin_data.email;
    let user_password = signin_data.password;

    // Perform database query to check if the user exists and validate the password
    // "SELECT id, email, password, first_name, last_name FROM users WHERE email = $1"
    let user: Option<AuthUser> = query_as!(
        AuthUser,
        "SELECT id, email,   username, phone_number, password_hash, verified, created_at, updated_at FROM users WHERE email = $1",
        user_email
    )
        .fetch_optional(&*pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

    if let Some(user) = user {
        // print!("user {:?}", user);
        if let (Some(id), Some(email), Some(phone_number), Some(password_hash)) =
            (user.id, user.email, user.phone_number, user.password_hash)
        {
            if bcrypt::verify(&user_password, &password_hash).expect("Failed to verify password") {
                // let jwt_secret = "CfLTk9J0MA3jBF3/zuE4VUyN7djM2KMPy4otUpbkbE8=";
                // let expiration = Utc::now() + Duration::hours(1);

                // let full_name = user.username.unwrap_or_else(|| "Unknown".to_string());
                let username = user.username.unwrap_or_else(|| "Unknown".to_string());
                // let profile_picture = user.profile_picture.unwrap_or_else(|| "Unknown".to_string());
                let verified = Some(user.verified).is_some();
                let created_at = if let Some(created_at) = user.created_at {
                    created_at
                } else {
                    Utc::now().naive_utc() // Default value if user.created_at is None
                };

                let updated_at = if let Some(updated_at) = user.updated_at {
                    updated_at
                } else {
                    Utc::now().naive_utc() // Default value if user.updated_at is None
                };

                let my_claims = Claims {
                    email: email.to_owned(),
                    verified,
                    // full_name,
                    username,
                    // profile_picture,
                    user_id: id.to_owned(),
                    phone_number: phone_number.to_owned(),
                    created_at,
                    updated_at,
                    exp: Utc::now()
                    .checked_add_signed(Duration::hours(24))
                    .expect("Invalid expiration time")
                    .timestamp(),
                };

                // let token = match encode(
                //     &Header::default(),
                //     &my_claims,
                //     &EncodingKey::from_secret(b"CfLTk9J0MA3jBF3/zuE4VUyN7djM2KMPy4otUpbkbE8="),
                // ) {
                //     Ok(t) => t,
                //     Err(_) => panic!(), // in practice you would return the error
                // };

                let secret_key = env::var("JWT_SECRET_KEY").map_err(|_| AuthError::MissingSecretKey)?;
                let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret_key.as_bytes()))
                    .map_err(|_| AuthError::JWTEncodingError)?;

                // Return success response with token
                return Ok(Json(json!({
                    "status": "success",
                    "message": "Sign-in successful",
                    "token": token
                })));
            } else {
                // Passwords do not match, return an error response
                return Ok(Json(json!({
                    "status": "error",
                    "message": "Invalid credentials"
                })));
            }
        } else {
            println!("Password not verified!");
            // Passwords do not match, return an error response
            return Ok(Json(json!({
                "status": "error",
                "message": "Invalid credentials"
            })));
        }
    }

    // If user doesn't exist or password is invalid, return an error response
    Ok(Json(json!({
        "status": "error",
        "message": "Invalid credentials"
    })))
}

#[derive(Debug)]
pub enum AuthError {
    DatabaseError(String),
    HashVerificationError(String),
    MissingSecretKey,
    JWTEncodingError,
    InvalidCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::DatabaseError(err) => Json(json!({
                "status": "error",
                "message": format!("Database error: {}", err)
            }))
            .into_response(),
            AuthError::HashVerificationError(err) => Json(json!({
                "status": "error",
                "message": format!("Hash verification error: {}", err)
            }))
            .into_response(),
            AuthError::MissingSecretKey => Json(json!({
                "status": "error",
                "message": "Missing JWT secret key"
            }))
            .into_response(),
            AuthError::JWTEncodingError => Json(json!({
                "status": "error",
                "message": "Error encoding JWT"
            }))
            .into_response(),
            AuthError::InvalidCredentials => Json(json!({
                "status": "error",
                "message": "Invalid credentials"
            }))
            .into_response(),
        }
    }
}

pub async fn sign_out(
    State(pool): State<Arc<PgPool>>,
    Json(signin_data): Json<SignInData>,
) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Invalid credentials"
    }))
}
