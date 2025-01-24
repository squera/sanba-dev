use crate::authentication::{create_jwt, password::verify_password};
use diesel::prelude::*;
use domain::models::{full_tables::User, others::LoginRequest};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

pub fn login_user(req: LoginRequest) -> Result<String, ApiError> {
    use domain::schema::user;

    let result_user = user::table
        .filter(user::email.eq(&req.email))
        .select(User::as_select())
        .first::<User>(&mut establish_connection())
        .optional()?;

    match result_user {
        Some(user) => {
            let psw_check = verify_password(&req.password, &user.password);

            match psw_check {
                Ok(true) => {
                    match create_jwt(user.person_id) {
                        Ok(token) => Ok(token),
                        Err(err) => Err(ApiError {
                            http_status: Status::InternalServerError,
                            error_code: 123, // TODO organizzare i codici di errore
                            error_type: ApiErrorType::ApplicationError,
                            message: format!(
                                "Error - Unable to create JWT token - {}",
                                err.to_string()
                            ),
                        }),
                    }
                }
                Ok(false) => {
                    return Err(ApiError {
                        http_status: Status::Unauthorized,
                        error_code: 123, // TODO organizzare i codici di errore
                        error_type: ApiErrorType::AuthenticationError,
                        message: format!("Error - Password not valid"),
                    });
                }
                Err(err) => {
                    return Err(ApiError {
                        http_status: Status::InternalServerError,
                        error_code: 123, // TODO organizzare i codici di errore
                        error_type: ApiErrorType::ApplicationError,
                        message: format!("Error - Unable to verify password - {}", err.to_string()),
                    });
                }
            }
        }
        None => {
            return Err(ApiError {
                http_status: Status::Unauthorized,
                error_code: 123, // TODO organizzare i codici di errore
                error_type: ApiErrorType::AuthenticationError,
                message: format!("Error - No user with email {} found", &req.email),
            });
        }
    }
}

pub fn extend_token(id: i64) -> Result<String, ApiError> {
    match create_jwt(id) {
        Ok(token) => Ok(token),
        Err(err) => Err(ApiError {
            http_status: Status::InternalServerError,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::ApplicationError,
            message: format!("Error - Unable to create JWT token - {}", err.to_string()),
        }),
    }
}
