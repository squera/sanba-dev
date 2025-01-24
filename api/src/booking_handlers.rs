use application::{
    authentication::JWT,
    db_entities::booking::{
        create::authorize_create_booking,
        delete::authorize_delete_booking_and_event,
        read::{find_booking, list_bookings},
        update::authorize_update_booking_and_event,
    },
};
use domain::models::others::{BookingWithEvent, NewBookingData};
use rocket::{delete, get, post, put, serde::json::Json};
use shared::response_models::ApiError;

/// Inserisce una nuova prenotazione
///
/// Dopo i dovuti controlli sui dati, inserisce la nuova prenotazione e l'evento associato nel database.
///
/// ### Chi ha accesso:
/// - Un responsabile di una società sportiva
/// - Un allenatore
#[utoipa::path(
    context_path = "/booking",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Prenotazioni"],
    responses(
        (status = CREATED, description = "Prenotazione inserita con successo", body = BookingWithEvent, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/", data = "<booking>")]
pub fn create_booking_handler(
    key: Result<JWT, ApiError>,
    booking: Json<NewBookingData>,
) -> Result<Json<BookingWithEvent>, ApiError> {
    let key = key?;

    let res = authorize_create_booking(key.claims, booking.into_inner())?;
    Ok(Json(res))
}

/// Restituisce una prenotazione
///
/// Restituisce i dati della prenotazione e della partita o allenamento associati.
///
/// ### Chi ha accesso:
/// - Chiunque è loggato
#[utoipa::path(
    context_path = "/booking",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Prenotazioni"],
    responses(
        (status = OK, description = "Prenotazione trovata con successo", body = BookingWithEvent, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Prenotazione non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("booking_id" = i64, Path, description = "ID della prenotazione da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/<booking_id>")]
pub fn find_booking_handler(
    key: Result<JWT, ApiError>,
    booking_id: i64,
) -> Result<Json<BookingWithEvent>, ApiError> {
    let _key = key?;

    let res = find_booking(booking_id)?;
    Ok(Json(res))
}

/// Restituisce una lista di prenotazioni
///
/// Restituisce la lista di tutte le prenotazioni per il dato sport. Se lo sport non viene fornito la lista comprende tutti gli sport.
///
/// ### Chi ha accesso:
/// - Chiunque è loggato
#[utoipa::path(
    context_path = "/booking",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Prenotazioni"],
    responses(
        (status = OK, description = "Prenotazioni trovate con successo", body = [BookingWithEvent], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/list", data = "<sport>")]
pub fn list_bookings_handler(
    key: Result<JWT, ApiError>,
    sport: Option<String>,
) -> Result<Json<Vec<BookingWithEvent>>, ApiError> {
    let _key = key?;
    // TODO implementare la ricerca per sport
    let res = list_bookings()?;
    Ok(Json(res))
}

/// Aggiorna i dati di una prenotazione
///
/// Dopo i dovuti controlli sui dati, vengono aggiornati i dati della prenotazione.
/// NB: per rimuovere l'evento (partita/allenamento) associato alla prenotazione o per cambiarlo, è necessario usare
/// la route apposita per eliminare l'evento.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva della squadra coinvolta.
/// - Un allenatore della squadra coinvolta.
/// Nel caso di una partita, vale anche per la squadra ospite, nel caso venisse inserita.
#[utoipa::path(
    context_path = "/booking",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Prenotazioni"],
    responses(
        (status = OK, description = "Dati aggiornati con successo", body = BookingWithEvent, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("booking_id" = i64, Path, description = "ID della prenotazione da aggiornare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/<booking_id>", data = "<booking>")]
pub fn update_booking_handler(
    key: Result<JWT, ApiError>,
    booking_id: i64,
    booking: Json<NewBookingData>,
) -> Result<Json<BookingWithEvent>, ApiError> {
    let key = key?;

    let res = authorize_update_booking_and_event(key.claims, booking_id, booking.into_inner())?;
    Ok(Json(res))
}

/// Elimina una prenotazione
///
/// Viene eliminata la prenotazione, la partita o allenamento associati, le formazioni (nel caso di una partita),
/// le associazioni con i giocatori per questo evento (partita/allenamento) e le sessioni di registrazione associate a questa prenotazione.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva della squadra coinvolta.
/// - Un allenatore della squadra coinvolta.
/// Nel caso di una partita, vale anche per la squadra ospite, nel caso venisse inserita.
#[utoipa::path(
    context_path = "/booking",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Prenotazioni"],
    responses(
        (status = OK, description = "Prenotazione eliminata con successo", body = BookingWithEvent, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Prenotazione non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("booking_id" = i64, Path, description = "ID della prenotazione da eliminare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<booking_id>")]
pub fn delete_booking_handler(
    key: Result<JWT, ApiError>,
    booking_id: i64,
) -> Result<Json<BookingWithEvent>, ApiError> {
    let key = key?;

    let res = authorize_delete_booking_and_event(key.claims, booking_id)?;
    Ok(Json(res))
}
