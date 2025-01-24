use std::collections::HashSet;

use diesel::prelude::*;
use domain::models::{
    full_tables::{CameraSession, RecordingSession},
    others::{RecordingSessionData, RecordingSessionWithCameras},
    WithId,
};
use infrastructure::establish_connection;
use shared::response_models::ApiError;

use crate::db_entities::recording_session::read::find_recording_session;

pub fn update_recording_session_and_cameras(
    session_id: i64,
    recording_session_data: RecordingSessionData,
) -> Result<RecordingSessionWithCameras, ApiError> {
    use domain::schema::camera_session;

    let connection = &mut establish_connection();

    let session_to_update = find_recording_session(session_id)?;

    update_recording_session(
        recording_session_data
            .recording_session
            .to_identified(session_id),
    )?;

    let camera_differences = calculate_differences(
        &session_to_update
            .cameras
            .iter()
            .map(|c| c.id)
            .collect::<Vec<_>>(),
        &recording_session_data.camera_ids,
    );

    // Aggiunta delle camere mancanti
    diesel::insert_into(camera_session::table)
        .values(
            camera_differences
                .0
                .iter()
                .map(|camera_id| CameraSession {
                    session_id,
                    camera_id: *camera_id,
                })
                .collect::<Vec<_>>(),
        )
        .execute(connection)?;

    // Rimozione delle camere in eccesso
    diesel::delete(
        camera_session::table
            .filter(camera_session::session_id.eq(&session_id))
            .filter(camera_session::camera_id.eq_any(&camera_differences.1)),
    )
    .execute(connection)?;

    let res = find_recording_session(session_id)?;
    return Ok(res);
}

pub fn update_recording_session(
    new_recording_session: RecordingSession,
) -> Result<RecordingSession, ApiError> {
    let connection = &mut establish_connection();

    let updated_session = new_recording_session.save_changes::<RecordingSession>(connection)?;

    return Ok(updated_session);
}

/// Calcola gli elementi da aggiungere e da rimuovere per trasformare il vettore A nel vettore B.
///
/// # Parametri
/// - `a`: Vettore di partenza.
/// - `b`: Vettore di destinazione.
///
/// # Ritorna
/// Una tupla contenente due vettori:
/// - Il primo con gli elementi da aggiungere.
/// - Il secondo con gli elementi da rimuovere.
fn calculate_differences(a: &[i64], b: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let set_a: HashSet<_> = a.iter().cloned().collect();
    let set_b: HashSet<_> = b.iter().cloned().collect();

    let to_add: Vec<_> = set_b.difference(&set_a).cloned().collect();
    let to_remove: Vec<_> = set_a.difference(&set_b).cloned().collect();

    (to_add, to_remove)
}
