use application::{
    authentication::JWT,
    db_entities::booking::game::{
        delete::delete_game,
        formation::{
            create::create_formation_player_tags, delete::delete_formation_players,
            read::get_formation_player_list,
        },
    },
};
use domain::models::{
    full_tables::Game,
    others::{FormationPlayerTagsData, FormationPlayerWithTags},
};
use rocket::{delete, get, post, serde::json::Json};
use shared::response_models::ApiError;

/// Restituisce una formazione
///
/// Restituisce i dati della formazione e dei giocatori associati.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva coinvolta nella partita.
/// - Un allenatore della squadra coinvolta nella partita.
/// - Un giocatore coinvolto nella formazione.
#[utoipa::path(
    context_path = "/game",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Partite"],
    responses(
        (status = OK, description = "Formazione trovata con successo", body = [FormationPlayerWithTags], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Formazione non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("game_id" = i64, Path, description = "ID della partita"),
        ("formation_id" = i64, Path, description = "ID della formazione da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/<game_id>/formation/<formation_id>")]
pub fn find_formation_handler(
    key: Result<JWT, ApiError>,
    game_id: i64,
    formation_id: i64,
) -> Result<Json<Vec<FormationPlayerWithTags>>, ApiError> {
    let _key = key?;

    // TODO controllare che la formazione sia associata alla partita corretta
    let res = get_formation_player_list(formation_id)?;
    Ok(Json(res))
}

/// Aggiunge giocatori a una formazione
///
/// Dopo i dovuti controlli sui dati, vengono inseriti i giocatori nella formazione.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva coinvolta nella partita.
/// - Un allenatore della squadra coinvolta nella partita.
#[utoipa::path(
    context_path = "/game",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Partite"],
    responses(
        (status = OK, description = "Dati aggiornati con successo", body = [FormationPlayerWithTags], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("game_id" = i64, Path, description = "ID della partita"),
        ("formation_id" = i64, Path, description = "ID della formazione da modificare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<game_id>/formation/<formation_id>", data = "<formation>")]
pub fn add_players_to_formation_handler(
    key: Result<JWT, ApiError>,
    game_id: i64,
    formation_id: i64,
    formation: Json<Vec<FormationPlayerTagsData>>,
) -> Result<Json<Vec<FormationPlayerWithTags>>, ApiError> {
    let _key = key?;

    // TODO controllare che la formazione sia associata alla partita corretta
    let res = create_formation_player_tags(formation_id, formation.into_inner())?;
    Ok(Json(res))
}

/// Elimina giocatori da una formazione
///
/// Elimina giocatori da una formazione
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva coinvolta nella partita.
/// - Un allenatore della squadra coinvolta nella partita.
#[utoipa::path(
    context_path = "/game",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Partite"],
    responses(
        (status = OK, description = "Formazione eliminata con successo", body = [FormationPlayerWithTags], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Formazione non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("game_id" = i64, Path, description = "ID della partita"),
        ("formation_id" = i64, Path, description = "ID della formazione da eliminare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<game_id>/formation/<formation_id>", data = "<player_list>")]
pub fn delete_players_from_formation_handler(
    key: Result<JWT, ApiError>,
    game_id: i64,
    formation_id: i64,
    player_list: Json<Vec<i64>>,
) -> Result<Json<Vec<FormationPlayerWithTags>>, ApiError> {
    let _key = key?;

    // TODO controllare che la formazione sia associata alla partita corretta
    let res = delete_formation_players(formation_id, player_list.into_inner())?;
    Ok(Json(res))
}

/// Elimina una partita
///
/// Viene eliminata la partita e le formazioni associate, ma non la prenotazione.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva coinvolta nella partita.
/// - Un allenatore della squadra coinvolta nella partita.
#[utoipa::path(
    context_path = "/game",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Partite"],
    responses(
        (status = OK, description = "Partita eliminata con successo", body = Game, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Partita non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("game_id" = i64, Path, description = "ID della partita"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<game_id>")]
pub fn delete_game_handler(
    key: Result<JWT, ApiError>,
    game_id: i64,
) -> Result<Json<Game>, ApiError> {
    let _key = key?;

    let res = delete_game(game_id)?;
    Ok(Json(res))
}
