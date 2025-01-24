use application::authentication::JWT;
use domain::models::full_tables::{Screenshot, Video};
use domain::models::others::{NewClip, NewScreenshot, NewTimestamp, UserList};
use rocket::{delete, get, post, serde::json::Json};
use shared::response_models::ApiError;

/// Restituisce la lista dei video per una prenotazione
///
/// Restituisce la lista con tutti i video registrati durante la partita o allenamento associati alla prenotazione.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
/// - Un giocatore della squadra
#[utoipa::path(
    context_path = "/video",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Dati registrati"],
    responses(
        (status = OK, description = "Video della prenotazione trovati con successo", body = [Video], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("booking_id" = i64, Path, description = "ID della prenotazione da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/list-by-booking/<booking_id>")]
pub fn list_videos_by_booking_handler(
    key: Result<JWT, ApiError>,
    booking_id: i64,
) -> Result<String, ApiError> {
    todo!()
}

/// Restituisce un video
///
/// Restituisce un video dato il suo ID.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
/// - Un giocatore
/// - Chiunque altro abbia l'accesso al video in lettura
#[utoipa::path(
    context_path = "/video",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Dati registrati"],
    responses(
        (status = OK, description = "Video trovato con successo", body = Video, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Video non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("video_id" = i64, Path, description = "ID del video da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/<video_id>")]
pub fn find_video_handler(key: Result<JWT, ApiError>, video_id: i64) -> Result<String, ApiError> {
    todo!()
}

/// Elimina un video
///
/// Elimina un video dato il suo ID.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
/// - Un giocatore
/// - Chiunque altro abbia l'accesso al video in scrittura
#[utoipa::path(
    context_path = "/video",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Dati registrati"],
    responses(
        (status = OK, description = "Video eliminato con successo", body = Video, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Video non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("video_id" = i64, Path, description = "ID del video da eliminare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<video_id>")]
pub fn delete_video_handler(key: Result<JWT, ApiError>, video_id: i64) -> Result<String, ApiError> {
    todo!()
}

/// Inserisce un nuovo screenshot associato al video
///
/// Viene inserito un nuovo screenshot associato al video specificato.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
/// - Un giocatore
/// - Chiunque altro abbia l'accesso al video in scrittura
#[utoipa::path(
    context_path = "/video",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Dati registrati"],
    responses(
        (status = CREATED, description = "Screenshot inserito con successo", body = String),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Video non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("video_id" = i64, Path, description = "ID del video da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<video_id>/screenshot", data = "<screenshot>")]
pub fn create_screenshot_handler(
    key: Result<JWT, ApiError>,
    video_id: i64,
    screenshot: Json<NewScreenshot>,
) -> Result<String, ApiError> {
    todo!()
}

/// Elimina uno screenshot associato a un video
///
/// Viene eliminato lo screenshot associato al video specificato.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
/// - Un giocatore
/// - Chiunque altro abbia l'accesso al video in scrittura
#[utoipa::path(
    context_path = "/video",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Dati registrati"],
    responses(
        (status = OK, description = "Screenshot eliminato con successo", body = Screenshot, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Video non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("video_id" = i64, Path, description = "ID del video da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<video_id>/screenshot/<screenshot_id>")]
pub fn delete_screenshot_handler(
    key: Result<JWT, ApiError>,
    video_id: i64,
    screenshot_id: i64,
) -> Result<String, ApiError> {
    todo!()
}

/// Inserisce un nuovo timestamp associato al video
///
/// Viene inserito un nuovo timestamp associato al video specificato.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
/// - Un giocatore
/// - Chiunque altro abbia l'accesso al video in scrittura
#[utoipa::path(
    context_path = "/video",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Dati registrati"],
    responses(
        (status = CREATED, description = "Timestamp inserito con successo", body = String),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Video non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("video_id" = i64, Path, description = "ID del video da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<video_id>/timestamp", data = "<timestamp>")]
pub fn create_timestamp_handler(
    key: Result<JWT, ApiError>,
    video_id: i64,
    timestamp: Json<NewTimestamp>,
) -> Result<String, ApiError> {
    todo!()
}

/// Elimina un timestamp associato a un video
///
/// Viene eliminato lo timestamp associato al video specificato.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
/// - Un giocatore
/// - Chiunque altro abbia l'accesso al video in scrittura
#[utoipa::path(
    context_path = "/video",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Dati registrati"],
    responses(
        (status = OK, description = "Timestamp eliminato con successo", body = Screenshot, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Video non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("video_id" = i64, Path, description = "ID del video da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<video_id>/timestamp/<timestamp_id>")]
pub fn delete_timestamp_handler(
    key: Result<JWT, ApiError>,
    video_id: i64,
    timestamp_id: i64,
) -> Result<String, ApiError> {
    todo!()
}

/// Inserisce una nuova clip associata a un video
///
/// Viene salvata una clip relativa al video specificato.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
/// - Un giocatore
/// - Chiunque altro abbia l'accesso al video in scrittura
#[utoipa::path(
    context_path = "/video",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Dati registrati"],
    responses(
        (status = CREATED, description = "Clip inserita con successo", body = String),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Video non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("video_id" = i64, Path, description = "ID del video da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<video_id>/clip", data = "<clip>")]
pub fn create_clip_handler(
    key: Result<JWT, ApiError>,
    video_id: i64,
    clip: Json<NewClip>,
) -> Result<String, ApiError> {
    todo!()
}

/// Permette di condividere un video con altri utenti
///
/// Gli utenti specificati vengono aggiunti alla lista di persone che hanno l'accesso al video.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
/// - Un giocatore
/// - Chiunque altro abbia l'accesso al video per condividere
#[utoipa::path(
    context_path = "/video",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Dati registrati"],
    responses(
        (status = OK, description = "Condivisione avvenuta con successo", body = String),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Video non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("video_id" = i64, Path, description = "ID del video da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<video_id>/share", data = "<users>")]
pub fn share_video_handler(
    key: Result<JWT, ApiError>,
    video_id: i64,
    users: Json<UserList>,
) -> Result<String, ApiError> {
    todo!()
}