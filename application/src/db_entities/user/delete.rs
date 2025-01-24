use diesel::prelude::*;
use domain::models::full_tables::User;
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{person_checks::is_administrator, user_checks::is_club_responsible},
};

pub fn authorize_delete_user(requesting_user: Claims, user_id: i64) -> Result<User, ApiError> {
    use crate::authorization::user_checks::is_same_user;

    if is_same_user(requesting_user.subject_id, user_id)
        || is_administrator(requesting_user.subject_id)?
    {
        return delete_user(user_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to delete user {}",
                requesting_user.subject_id, user_id
            ),
        });
    }
}

pub(crate) fn delete_user(id: i64) -> Result<User, ApiError> {
    use domain::schema::user;

    if is_club_responsible(id, None, true)? {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::ApplicationError,
            message: format!(
                "Error - User {} is responsible for a club and cannot be deleted",
                id
            ),
        });
    } else {
        let connection = &mut establish_connection();

        let user_to_delete = user::table
            .filter(user::person_id.eq(id))
            .first::<User>(connection)?;

        diesel::delete(user::table.filter(user::person_id.eq(id))).execute(connection)?;

        return Ok(user_to_delete);
    }
}
