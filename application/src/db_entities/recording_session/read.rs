use chrono::offset;
use diesel::prelude::*;
use domain::{
    models::{
        full_tables::{Camera, RecordingSession},
        others::RecordingSessionWithCameras,
    },
    schema::camera,
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{authentication::Claims, authorization::booking_checks::can_read_recording_session};

pub fn authorize_find_recording_session(
    requesting_user: Claims,
    session_id: i64,
) -> Result<RecordingSessionWithCameras, ApiError> {
    if can_read_recording_session(requesting_user.subject_id, session_id)? {
        return find_recording_session(session_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to read recording session {}",
                requesting_user.subject_id, session_id
            ),
        });
    }
}

pub fn find_recording_session(session_id: i64) -> Result<RecordingSessionWithCameras, ApiError> {
    use domain::schema::{camera_session, recording_session};

    let connection = &mut establish_connection();

    let recording_session = recording_session::table
        .filter(recording_session::id.eq(session_id))
        .select(RecordingSession::as_select())
        .get_result(connection)?;

    let cameras = camera_session::table
        .filter(camera_session::session_id.eq(session_id))
        .inner_join(camera::table)
        .select(Camera::as_select())
        .load(connection)?;

    return Ok(RecordingSessionWithCameras {
        recording_session,
        cameras,
    });
}

pub fn authorize_list_recording_sessions_by_booking(
    requesting_user: Claims,
    booking_id: i64,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<RecordingSessionWithCameras>, ApiError> {
    if can_read_recording_session(requesting_user.subject_id, booking_id)? {
        return list_recording_sessions_by_booking(booking_id, limit, offset);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to read recording sessions for booking {}",
                requesting_user.subject_id, booking_id
            ),
        });
    }
}

pub fn list_recording_sessions_by_booking(
    booking_id: i64,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<RecordingSessionWithCameras>, ApiError> {
    use domain::schema::{camera_session, recording_session};

    let connection = &mut establish_connection();

    let mut query = recording_session::table
        .filter(recording_session::booking_id.eq(booking_id))
        .select(RecordingSession::as_select())
        .into_boxed();

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    let mut recording_sessions: Vec<RecordingSessionWithCameras> = query
        .load(connection)?
        .into_iter()
        .map(|recording_session| RecordingSessionWithCameras {
            recording_session,
            cameras: Vec::new(),
        })
        .collect();

    let cameras: Vec<(i64, Camera)> = camera_session::table
        .filter(
            camera_session::session_id
                .eq_any(recording_sessions.iter().map(|s| s.recording_session.id)),
        )
        .inner_join(camera::table)
        .select((camera_session::session_id, Camera::as_select()))
        .load(connection)?;

    merge_cameras(&mut recording_sessions, &cameras);

    return Ok(recording_sessions);
}

fn merge_cameras(
    recording_sessions: &mut Vec<RecordingSessionWithCameras>,
    cameras: &[(i64, Camera)],
) {
    // Creiamo una mappa da player_id a un vettore di rfid_tag_id per accesso rapido
    let mut camera_map: std::collections::HashMap<i64, Vec<Camera>> =
        std::collections::HashMap::new();

    for camera in cameras {
        camera_map
            .entry(camera.0)
            .or_default()
            .push(camera.1.clone());
    }

    // Aggiorniamo il campo rfid_tag_ids per ogni TrainingPlayerWithTags
    for session in recording_sessions {
        if let Some(cameras) = camera_map.get(&session.recording_session.id) {
            session.cameras = cameras.clone();
        } else {
            session.cameras = Vec::new(); // Nessun tag associato
        }
    }
}
