use diesel::prelude::*;
use domain::{
    models::{
        full_tables::{Camera, RecordingSession},
        others::RecordingSessionWithCameras,
    },
    schema::camera,
};
use infrastructure::establish_connection;
use shared::response_models::ApiError;

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

pub fn list_recording_sessions_by_booking(
    booking_id: i64,
) -> Result<Vec<RecordingSessionWithCameras>, ApiError> {
    use domain::schema::{camera_session, recording_session};

    let connection = &mut establish_connection();

    let mut recording_sessions: Vec<RecordingSessionWithCameras> = recording_session::table
        .filter(recording_session::booking_id.eq(booking_id))
        .select(RecordingSession::as_select())
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
