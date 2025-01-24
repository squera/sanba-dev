use diesel::prelude::*;
use domain::{models::others::RecordingSessionWithCameras, schema::recording_session};
use infrastructure::establish_connection;
use shared::response_models::ApiError;

use crate::db_entities::recording_session::read::find_recording_session;

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
