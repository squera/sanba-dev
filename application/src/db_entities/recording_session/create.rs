use diesel::prelude::*;
use diesel::result::Error;
use domain::models::{
    full_tables::{CameraSession, RecordingSession},
    insertions::NewRecordingSession,
    others::{RecordingSessionData, RecordingSessionWithCameras},
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};
use validator::Validate;

use crate::db_entities::recording_session::read::find_recording_session;

pub fn create_recording_session_with_cameras(
    session_data: RecordingSessionData,
) -> Result<RecordingSessionWithCameras, ApiError> {
    use domain::schema::camera_session;

    session_data.validate()?;

    let recording_session = create_recording_session(session_data.recording_session)?;

    let camera_associations: Vec<CameraSession> = session_data
        .camera_ids
        .into_iter()
        .map(|camera_id| CameraSession {
            camera_id: camera_id,
            session_id: recording_session.id,
        })
        .collect();

    let connection = &mut establish_connection();

    diesel::insert_into(camera_session::table)
        .values(&camera_associations)
        .execute(connection)?;

    let res = find_recording_session(recording_session.id)?;
    return Ok(res);
}

/// Inserisce una nuova squadra nel database e la restituisce.
fn create_recording_session(
    new_session: NewRecordingSession,
) -> Result<RecordingSession, ApiError> {
    use domain::schema::recording_session;

    let connection = &mut establish_connection();

    let inserted_session: RecordingSession =
        match connection.transaction::<_, Error, _>(|connection| {
            diesel::insert_into(recording_session::table)
                .values(&new_session)
                .execute(connection)?;

            // NB: questo metodo per ottenere in ritorno la sessione di registrazione inserita si affida al fatto che gli id siano autoincrementali.
            // Purtroppo attualmente è l'unico modo con MySQL per ottenere l'id della sessione di registrazione appena inserita.
            // Valutare il passaggio a PostgreSQL per utilizzare il metodo `returning` di Diesel o attendere un supporto a MariaDB.
            recording_session::table
                .order(recording_session::id.desc())
                .first(connection)
        }) {
            Ok(p) => p,
            Err(err) => {
                return Err(ApiError {
                    http_status: Status::InternalServerError,
                    error_code: 123,
                    error_type: ApiErrorType::ApplicationError,
                    message: format!("Error while inserting new recording session - {}", err),
                })
            }
        };

    return Ok(inserted_session);
}
