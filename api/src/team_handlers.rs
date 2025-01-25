use application::{
    authentication::JWT,
    db_entities::team::{
        create::authorize_create_team,
        delete::authorize_delete_team,
        read::{find_team, list_teams},
        update::authorize_update_team,
    },
};
use domain::models::{full_tables::Team, insertions::NewTeam};
use rocket::{delete, get, post, put, serde::json::Json};
use shared::response_models::ApiError;

/// Inserisce una nuova squadra
///
/// Dopo i dovuti controlli sui dati, inserisce la nuova squadra nel database.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva specificata nei dati della nuova squadra
#[utoipa::path(
    context_path = "/team",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Squadre"],
    responses(
        (status = CREATED, description = "Squadra inserita con successo", body = Team, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/", data = "<team>")]
pub fn create_team_handler(
    key: Result<JWT, ApiError>,
    team: Json<NewTeam>,
) -> Result<Json<Team>, ApiError> {
    let key = key?;

    let res = authorize_create_team(key.claims, team.into_inner())?;
    Ok(Json(res))
}

/// Restituisce una squadra
///
/// Restituisce una squadra dato il suo ID
///
/// ### Chi ha accesso:
/// - Chiunque è loggato
#[utoipa::path(
    context_path = "/team",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Squadre"],
    responses(
        (status = OK, description = "Squadra trovata con successo", body = Team, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Squadra non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("team_id" = i64, Path, description = "ID della squadra da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/<team_id>")]
pub fn find_team_handler(key: Result<JWT, ApiError>, team_id: i64) -> Result<Json<Team>, ApiError> {
    let _key = key?;

    let res = find_team(team_id)?;
    Ok(Json(res))
}

/// Restituisce una lista di squadre
///
/// Restituisce la lista di tutte le squadre che praticano il dato sport. Se lo sport non viene fornito la lista comprende tutti gli sport
///
/// ### Chi ha accesso:
/// - Chiunque è loggato
#[utoipa::path(
    context_path = "/team",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Squadre"],
    responses(
        (status = OK, description = "Squadre trovate con successo", body = [Team], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/list", data = "<sport>")]
pub fn list_teams_handler(
    key: Result<JWT, ApiError>,
    sport: Option<String>,
) -> Result<Json<Vec<Team>>, ApiError> {
    let _key = key?;
    // TODO aggiungere filtro per sport, società sportiva, paginazione
    let res = list_teams()?;
    Ok(Json(res))
}

/// Aggiorna i dati di una squadra
///
/// Dopo i dovuti controlli sui dati, vengono aggiornati i dati della squadra.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
/// - Un allenatore della squadra
#[utoipa::path(
    context_path = "/team",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Squadre"],
    responses(
        (status = OK, description = "Dati aggiornati con successo", body = Team, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("team_id" = i64, Path, description = "ID della squadra da aggiornare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/<team_id>", data = "<team>")]
pub fn update_team_handler(
    key: Result<JWT, ApiError>,
    team_id: i64,
    team: Json<NewTeam>,
) -> Result<Json<Team>, ApiError> {
    let key = key?;

    let res = authorize_update_team(key.claims, team_id, team.into_inner())?;
    Ok(Json(res))
}

/// Elimina una squadra
///
/// Se tutte le informazioni in relazione con questa squadra sono già state eliminate, viene eliminata la squadra.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva
#[utoipa::path(
    context_path = "/team",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Squadre"],
    responses(
        (status = OK, description = "Squadra eliminata con successo", body = [Team], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Squadra non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("team_id" = i64, Path, description = "ID della squadra da eliminare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<team_id>")]
pub fn delete_team_handler(
    key: Result<JWT, ApiError>,
    team_id: i64,
) -> Result<Json<Team>, ApiError> {
    let key = key?;

    let res = authorize_delete_team(key.claims, team_id)?;
    Ok(Json(res))
}
