use application::{
    authentication::JWT,
    db_entities::club::{
        create::create_club,
        delete::authorize_delete_club,
        read::{find_club, list_clubs},
        update::{
            authorize_add_club_responsible, authorize_remove_club_responsible,
            authorize_update_club,
        },
    },
};
use domain::models::{full_tables::SportsClub, insertions::NewSportsClub, WithId};
use rocket::{delete, get, post, put, response::status::Created, serde::json::Json};
use shared::response_models::ApiError;

/// Inserisce una nuova società sportiva
///
/// Dopo i dovuti controlli sui dati, inserisce la nuova società nel database.
/// L'utente che ha inviato la richiesta viene registrato come referente della società.
///
/// ### Chi ha accesso:
/// - Chiunque è loggato
#[utoipa::path(
    context_path = "/club",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Società sportive"],
    responses(
        (status = CREATED, description = "Società sportiva inserita con successo", body = String),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/", data = "<club>")]
pub fn create_club_handler(
    key: Result<JWT, ApiError>,
    club: Json<SportsClub>,
) -> Result<Created<Json<SportsClub>>, ApiError> {
    let key = key?;

    let res = create_club(club.into_inner(), key.claims.subject_id)?;
    Ok(Created::new("").body(Json(res)))
    // TODO modificare per impostare correttamente la Location dell'header
}

/// Restituisce una società sportiva
///
/// Restituisce i dati della società sportiva dato il suo ID.
///
/// ### Chi ha accesso:
/// - Chiunque è loggato
#[utoipa::path(
    context_path = "/club",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Società sportive"],
    responses(
        (status = OK, description = "Socetà sportiva trovata con successo", body = SportsClub, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Soccietà sportiva non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("club_id" = String, Path, description = "Partita IVA della società sportiva da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/<club_id>")]
pub fn find_club_handler(
    key: Result<JWT, ApiError>,
    club_id: String,
) -> Result<Json<SportsClub>, ApiError> {
    let _key = key?;

    let res = find_club(club_id)?;
    Ok(Json(res))
}

/// Restituisce una lista di società sportive
///
/// Restituisce tutte le società sportive salvate nel database
///
/// ### Chi ha accesso:
/// - Chiunque è loggato
#[utoipa::path(
    context_path = "/club",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Società sportive"],
    responses(
        (status = OK, description = "Società sportive trovate con successo", body = [SportsClub], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/list?<limit>&<offset>")]
pub fn list_clubs_handler(
    key: Result<JWT, ApiError>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Json<Vec<SportsClub>>, ApiError> {
    let _key = key?;

    let res = list_clubs(limit, offset)?;
    Ok(Json(res))
}

/// Aggiorna i dati di una società sportiva
///
/// Dopo i dovuti controlli sui dati, vengono aggiornati i dati della società
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
#[utoipa::path(
    context_path = "/club",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Società sportive"],
    responses(
        (status = OK, description = "Dati aggiornati con successo", body = SportsClub, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("club_id" = String, Path, description = "Partita IVA della società sportiva da aggiornare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/<club_id>", data = "<club>")]
pub fn update_club_handler(
    key: Result<JWT, ApiError>,
    club_id: String,
    club: Json<NewSportsClub>,
) -> Result<Json<SportsClub>, ApiError> {
    let key = key?;

    let res = authorize_update_club(key.claims, club.into_inner().to_identified(club_id))?;
    Ok(Json(res))
}

/// Aggiunge un nuovo utente come responsabile della società sportiva
///
/// Aggiunge un nuovo utente come responsabile della società sportiva
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
#[utoipa::path(
    context_path = "/club",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Società sportive"],
    responses(
        (status = OK, description = "Responsabile inserito con successo"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("club_id" = String, Path, description = "Partita IVA della società sportiva da aggiornare"),
        ("user_id" = i64, Path, description = "ID dell'utente da aggiungere come responsabile"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<club_id>/responsibles/<user_id>")]
pub fn add_club_responsible_handler(
    key: Result<JWT, ApiError>,
    club_id: String,
    user_id: i64,
) -> Result<(), ApiError> {
    let key = key?;

    authorize_add_club_responsible(key.claims, club_id, user_id)?;
    Ok(())
}

/// Rimuove un responsabile della società sportiva
///
/// Inserisce la data di fine del rapporto tra l'utente e la società sportiva
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
#[utoipa::path(
    context_path = "/club",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Società sportive"],
    responses(
        (status = OK, description = "Responsabile rimosso con successo"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("club_id" = String, Path, description = "Partita IVA della società sportiva da aggiornare"),
        ("user_id" = i64, Path, description = "ID dell'utente da rimuovere come responsabile"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<club_id>/responsibles/<user_id>")]
pub fn remove_club_responsible_handler(
    key: Result<JWT, ApiError>,
    club_id: String,
    user_id: i64,
) -> Result<(), ApiError> {
    let key = key?;

    authorize_remove_club_responsible(key.claims, club_id, user_id)?;
    Ok(())
}

/// Elimina una società sportiva
///
/// Se tutte le informazioni in relazione con questa società sportiva sono già state eliminate, viene eliminata la società sportiva.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
#[utoipa::path(
    context_path = "/club",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Società sportive"],
    responses(
        (status = OK, description = "Società sportiva eliminata con successo", body = SportsClub, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Società sportiva non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("club_id" = String, Path, description = "Partita IVA della società sportiva da eliminare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<club_id>")]
pub fn delete_club_handler(
    key: Result<JWT, ApiError>,
    club_id: String,
) -> Result<Json<SportsClub>, ApiError> {
    let key = key?;

    let res = authorize_delete_club(key.claims, club_id)?;
    Ok(Json(res))
}
