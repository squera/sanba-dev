use diesel::prelude::*;
use domain::{models::others::RecordingSessionWithCameras, schema::recording_session};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims, authorization::booking_checks::can_edit_delete_booking,
    db_entities::recording_session::read::find_recording_session,
};

pub fn authorize_delete_recording_session(
    requesting_user: Claims,
    session_id: i64,
) -> Result<RecordingSessionWithCameras, ApiError> {
    if can_edit_delete_booking(requesting_user.subject_id, session_id)? {
        return delete_recording_session(session_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to delete recording session {}",
                requesting_user.subject_id, session_id
            ),
        });
    }
}

pub fn delete_recording_session(session_id: i64) -> Result<RecordingSessionWithCameras, ApiError> {
    use domain::schema::camera_session;

    let connection = &mut establish_connection();

    let session_to_delete = find_recording_session(session_id)?;

    // Eliminazione delle associazioni tra camere e sessione di registrazione
    diesel::delete(camera_session::table.filter(camera_session::session_id.eq(&session_id)))
        .execute(connection)?;

    diesel::delete(recording_session::table.filter(recording_session::id.eq(&session_id)))
        .execute(connection)?;

    Ok(session_to_delete)
}
