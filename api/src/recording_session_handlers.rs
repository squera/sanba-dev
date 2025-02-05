use application::{
    authentication::JWT,
    db_entities::recording_session::{
        create::authorize_create_recording_session_with_cameras,
        delete::authorize_delete_recording_session,
        read::{authorize_find_recording_session, authorize_list_recording_sessions_by_booking},
        update::authorize_update_recording_session_and_cameras,
    },
};
use domain::models::others::{RecordingSessionData, RecordingSessionWithCameras};
use rocket::{delete, get, post, put, serde::json::Json};
use shared::response_models::ApiError;

/// Inserisce una nuova sessione di registrazione
///
/// Dopo i dovuti controlli sui dati, inserisce la nuova sessione di registrazione per la prenotazione specificata.
///
/// ### Chi ha accesso:
/// - Il responsabile delle società sportive delle squadre coinvolta nella prenotazione.
/// - Un allenatore delle squadre coinvolte nella prenotazione.
#[utoipa::path(
    context_path = "/recording-session",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Sessioni di registrazione"],
    responses(
        (status = CREATED, description = "Sessione di registrazione inserita con successo", body = RecordingSessionWithCameras, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/", data = "<recording_session>")]
pub fn create_recording_session_handler(
    key: Result<JWT, ApiError>,
    recording_session: Json<RecordingSessionData>,
) -> Result<Json<RecordingSessionWithCameras>, ApiError> {
    let key = key?;

    let res = authorize_create_recording_session_with_cameras(
        key.claims,
        recording_session.into_inner(),
    )?;
    Ok(Json(res))
}

/// Restituisce una sessione di registrazione
///
/// Restituisce i dati della sessione di registrazione.
///
/// ### Chi ha accesso:
/// - Il responsabile delle società sportive delle squadre coinvolta nella prenotazione.
/// - Un allenatore delle squadre coinvolte nella prenotazione.
/// - Un giocatore delle squadre coinvolte nella prenotazione.
#[utoipa::path(
    context_path = "/recording-session",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Sessioni di registrazione"],
    responses(
        (status = OK, description = "Sessione di registrazione trovata con successo", body = RecordingSessionWithCameras, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Sessione di registrazione non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("recording_session_id" = i64, Path, description = "ID della sessione di registrazione da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/<recording_session_id>")]
pub fn find_recording_session_handler(
    key: Result<JWT, ApiError>,
    recording_session_id: i64,
) -> Result<Json<RecordingSessionWithCameras>, ApiError> {
    let key = key?;

    let res = authorize_find_recording_session(key.claims, recording_session_id)?;
    Ok(Json(res))
}

/// Restituisce una lista di sessioni di registrazione che appartengono a una prenotazione
///
/// Viene restituita la lista di tutte le sessioni di registrazione che appartengono alla prenotazione specificata.
///
/// ### Chi ha accesso:
/// - Il responsabile delle società sportive delle squadre coinvolta nella prenotazione.
/// - Un allenatore delle squadre coinvolte nella prenotazione.
/// - Un giocatore delle squadre coinvolte nella prenotazione.
#[utoipa::path(
    context_path = "/recording-session",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Sessioni di registrazione"],
    responses(
        (status = OK, description = "Sessioni di registrazione trovate con successo", body = [RecordingSessionWithCameras], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("booking_id" = i64, Path, description = "ID della prenotazione di cui cercare le sessioni di registrazione"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/list-by-booking/<booking_id>?<limit>&<offset>")]
pub fn list_recording_sessions_by_booking_handler(
    key: Result<JWT, ApiError>,
    booking_id: i64,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Json<Vec<RecordingSessionWithCameras>>, ApiError> {
    let key = key?;

    let res = authorize_list_recording_sessions_by_booking(key.claims, booking_id, limit, offset)?;
    Ok(Json(res))
}

/// Aggiorna i dati di una sessione di registrazione
///
/// Dopo i dovuti controlli sui dati, vengono aggiornati i dati della sessione di registrazione.
///
/// ### Chi ha accesso:
/// - Il responsabile delle società sportive delle squadre coinvolta nella prenotazione.
/// - Un allenatore delle squadre coinvolte nella prenotazione.
#[utoipa::path(
    context_path = "/recording-session",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Sessioni di registrazione"],
    responses(
        (status = OK, description = "Dati aggiornati con successo", body = RecordingSessionWithCameras, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("recording_session_id" = i64, Path, description = "ID della sessione di registrazione da modificare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/<recording_session_id>", data = "<recording_session>")]
pub fn update_recording_session_handler(
    key: Result<JWT, ApiError>,
    recording_session_id: i64,
    recording_session: Json<RecordingSessionData>,
) -> Result<Json<RecordingSessionWithCameras>, ApiError> {
    let key = key?;

    let res = authorize_update_recording_session_and_cameras(
        key.claims,
        recording_session_id,
        recording_session.into_inner(),
    )?;
    Ok(Json(res))
}

/// Elimina una sessione di registrazione
///
/// Viene eliminata la sessione di registrazione.
///
/// ### Chi ha accesso:
/// - Il responsabile delle società sportive delle squadre coinvolta nella prenotazione.
/// - Un allenatore delle squadre coinvolte nella prenotazione.
#[utoipa::path(
    context_path = "/recording-session",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Sessioni di registrazione"],
    responses(
        (status = OK, description = "Sessione di registrazione eliminata con successo", body = RecordingSessionWithCameras, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Sessione di registrazione non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("recording_session_id" = i64, Path, description = "ID della sessione di registrazione da eliminare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<recording_session_id>")]
pub fn delete_recording_session_handler(
    key: Result<JWT, ApiError>,
    recording_session_id: i64,
) -> Result<Json<RecordingSessionWithCameras>, ApiError> {
    let key = key?;

    let res = authorize_delete_recording_session(key.claims, recording_session_id)?;
    Ok(Json(res))
}
