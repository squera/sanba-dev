use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::Serialize;
use utoipa::ToSchema;

// Struttura per creare delle risposte con dati JSON e uno stato HTTP
// #[derive(Debug)]
// pub struct JsonResponse<T: Serialize> {
//     pub status: rocket::http::Status,
//     pub data: T,
// }

// impl<'r, T: Serialize> Responder<'r, 'static> for JsonResponse<T> {
//     fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
//         let body = serde_json::to_string(&self.data)
//             .map_err(|_| rocket::http::Status::InternalServerError)?;
//         Response::build()
//             .header(ContentType::JSON)
//             .status(self.status)
//             .sized_body(body.len(), Cursor::new(body))
//             .ok()
//     }
// }

// Struttura da restituire in caso di errori
// TODO documentare con utoipa
#[derive(Serialize, ToSchema, Debug)]
pub struct ApiError {
    pub error_code: u16,
    pub error_type: ApiErrorType,
    pub message: String,

    #[schema(ignore = true)]
    #[serde(skip)]
    pub http_status: Status, // Usato solo per l'implementazione di Responder
}

#[derive(Serialize, ToSchema, Debug)]
pub enum ApiErrorType {
    ApplicationError,
    AuthenticationError,
    AuthorizationError,
    DieselDatabaseError,
    None,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body =
            serde_json::to_string(&self).map_err(|_| rocket::http::Status::InternalServerError)?;
        Response::build()
            .header(ContentType::JSON)
            .status(self.http_status)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(value: diesel::result::Error) -> Self {
        return match value {
            diesel::result::Error::DatabaseError(kind, info) => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: format!(
                    "Database error: {:?}\nInformation: {:?}",
                    kind,
                    info.message()
                ),
                http_status: Status::BadRequest,
            },
            diesel::result::Error::NotFound => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: "Resource not found".to_string(),
                http_status: Status::NotFound,
            },
            diesel::result::Error::QueryBuilderError(e) => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: format!("Query builder error: {}", e),
                http_status: Status::InternalServerError,
            },
            diesel::result::Error::DeserializationError(e) => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: format!("Deserialization error: {}", e),
                http_status: Status::InternalServerError,
            },
            diesel::result::Error::SerializationError(e) => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: format!("Serialization error: {}", e),
                http_status: Status::InternalServerError,
            },
            diesel::result::Error::RollbackErrorOnCommit {
                rollback_error,
                commit_error,
            } => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: format!(
                    "Rollback error on commit: {}\nCommit error: {}",
                    rollback_error, commit_error
                ),
                http_status: Status::InternalServerError,
            },
            diesel::result::Error::AlreadyInTransaction => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: "Already in transaction: attempted to perform a no-transaction operation inside a transaction".to_string(),
                http_status: Status::InternalServerError,
            },
            diesel::result::Error::NotInTransaction => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: "Not in transaction: the opertion attempted requires a transaction".to_string(),
                http_status: Status::InternalServerError,
            },
            diesel::result::Error::BrokenTransactionManager => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: "Broken transaction manager".to_string(),
                http_status: Status::InternalServerError,
            },
            _ => ApiError {
                error_code: 123,
                error_type: ApiErrorType::DieselDatabaseError,
                message: "Diesel error".to_string(),
                http_status: Status::InternalServerError,
            },
        };
    }
}
