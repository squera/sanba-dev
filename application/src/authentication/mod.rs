pub mod password;

use chrono::Utc;
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};
use shared::response_models::{ApiError, ApiErrorType};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i64, // ID dell'utente che si autentica
    exp: usize,          // scadenza del token in secondi
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

// TODO è possibile spostare nel pacchetto API visto che questo è codice relativo a Rocket?
#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ApiError> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                Outcome::Error((
                    Status::Unauthorized,
                    ApiError {
                        http_status: Status::Unauthorized,
                        error_code: 123, // TODO organizzare i codici di errore
                        error_type: ApiErrorType::AuthenticationError,
                        message: format!("Error validating JWT token - No token provided"),
                    },
                ))
            }
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        Outcome::Error((
                            Status::Unauthorized,
                            ApiError {
                                http_status: Status::Unauthorized,
                                error_code: 123, // TODO organizzare i codici di errore
                                error_type: ApiErrorType::AuthenticationError,
                                message: format!("Error validating JWT token - Expired Token"),
                            },
                        ))
                    }
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        Outcome::Error((
                            Status::Unauthorized,
                            ApiError {
                                http_status: Status::Unauthorized,
                                error_code: 123, // TODO organizzare i codici di errore
                                error_type: ApiErrorType::AuthenticationError,
                                message: format!("Error validating JWT token - Invalid Token"),
                            },
                        ))
                    }
                    _ => {
                        Outcome::Error((
                            Status::Unauthorized,
                            ApiError {
                                http_status: Status::Unauthorized,
                                error_code: 123, // TODO organizzare i codici di errore
                                error_type: ApiErrorType::AuthenticationError,
                                message: format!("Error validating JWT token - {}", err),
                            },
                        ))
                    }
                },
            },
        }
    }
}

pub fn create_jwt(id: i64) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let token_duration = env::var("JWT_DURATION_SECONDS")
        .expect("JWT_DURATION_SECONDS must be set.")
        .parse::<i64>()
        .expect("JWT_DURATION_SECONDS must be a number.");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(token_duration))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}
