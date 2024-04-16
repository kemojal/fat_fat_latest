use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeaderExtra},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestExt, TypedHeader,
};
use axum_extra::extract::Extract;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tower_http::auth::AuthorizationHeader;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub email: String,
    pub verified: bool,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthorizationHeaderRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .extract::<Option<TypedHeader<Authorization<Bearer>>>>()
            .await
            .map_err(|_| AuthorizationHeaderRejection)?
            .map(|a| a.0);

        if let Some(auth) = auth_header {
            let secret_key = "CfLTk9J0MA3jBF3/zuE4VUyN7djM2KMPy4otUpbkbE8=";
            let token_data = decode::<Claims>(
                &auth.token,
                &DecodingKey::from_secret(secret_key.as_bytes()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            )
            .map_err(|_| AuthorizationHeaderRejection)?;

            let authenticated_user = AuthenticatedUser {
                user_id: token_data.claims.user_id,
                email: token_data.claims.email,
                verified: token_data.claims.verified,
                username: token_data.claims.username,
                created_at: token_data.claims.created_at,
                updated_at: token_data.claims.updated_at,
            };

            Ok(authenticated_user)
        } else {
            Err(AuthorizationHeaderRejection)
        }
    }
}

struct AuthorizationHeaderRejection;

impl axum::response::IntoResponse for AuthorizationHeaderRejection {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::UNAUTHORIZED,
            "You are not authorized to access this resource",
        )
            .into_response()
    }
}