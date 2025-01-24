use application::{
    authentication::JWT,
    db_entities::booking::training::{
        create::create_training_player_tags,
        delete::{delete_training, delete_training_players},
        read::get_training_player_list,
    },
};
use domain::models::{
    full_tables::Training,
    others::{TrainingPlayerTagsData, TrainingPlayerWithTags},
};
use rocket::{delete, get, post, serde::json::Json};
use shared::response_models::ApiError;

/// Aggiunge dei giocatori ad un allenamento
///
/// Dopo i dovuti controlli sui dati, registra i giocatori come parte dell'allenamento.
/// NB: per modificare i dati di un giocatore già esistente, rimuoverlo e reinserirlo con i nuovi dati.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva della squadra coinvolta nell'allenamento.
/// - Un allenatore della squadra coinvolta nell'allenamento.
#[utoipa::path(
    context_path = "/training",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Allenamenti"],
    responses(
        (status = CREATED, description = "Lista giocatori inserita con successo", body = [TrainingPlayerWithTags], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Allenamento non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("training_id" = i64, Path, description = "ID dell'allenamento al quale associare la lista giocatori"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<training_id>/player-list", data = "<players_data>")]
pub fn add_training_player_list_handler(
    key: Result<JWT, ApiError>,
    training_id: i64,
    players_data: Json<Vec<TrainingPlayerTagsData>>,
) -> Result<Json<Vec<TrainingPlayerWithTags>>, ApiError> {
    let _key = key?;

    let res = create_training_player_tags(training_id, players_data.into_inner())?;
    Ok(Json(res))
}

/// Restituisce la lista dei giocatori partecipanti ad un allenamento
///
/// Restituisce la lista dei giocatori e dei tag a loro assegnati
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva della squadra coinvolta nell'allenamento.
/// - Un allenatore della squadra coinvolta nell'allenamento.
/// - Un giocatore della squadra coinvolta nell'allenamento.
#[utoipa::path(
    context_path = "/training",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Allenamenti"],
    responses(
        (status = OK, description = "Lista giocatori trovata con successo", body = [TrainingPlayerWithTags], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Allenamento non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("training_id" = i64, Path, description = "ID dell'allenamento"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/<training_id>/player-list")]
pub fn find_training_player_list_handler(
    key: Result<JWT, ApiError>,
    training_id: i64,
) -> Result<Json<Vec<TrainingPlayerWithTags>>, ApiError> {
    let _key = key?;

    let res = get_training_player_list(training_id)?;
    Ok(Json(res))
}

/// Elimina dei giocatori da un allenamento
///
/// Vengono eliminate le associazioni con i giocatori specificati.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva della squadra coinvolta nell'allenamento.
/// - Un allenatore della squadra coinvolta nell'allenamento.
#[utoipa::path(
    context_path = "/training",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Allenamenti"],
    responses(
        (status = OK, description = "Lista giocatori eliminata con successo", body = [TrainingPlayerWithTags], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Allenamento non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("training_id" = i64, Path, description = "ID dell'allenamento"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<training_id>/player-list", data = "<player_ids>")]
pub fn delete_training_player_list_handler(
    key: Result<JWT, ApiError>,
    training_id: i64,
    player_ids: Json<Vec<i64>>,
) -> Result<Json<Vec<TrainingPlayerWithTags>>, ApiError> {
    let _key = key?;

    let res = delete_training_players(training_id, player_ids.into_inner())?;
    Ok(Json(res))
}

/// Elimina un allenamento
///
/// Viene eliminato l'allenamento e la lista giocatori, ma non la prenotazione.
///
/// ### Chi ha accesso:
/// - Il responsabile della società sportiva coinvolta nell'allenamento.
/// - Un allenatore della squadra coinvolta nell'allenamento.
#[utoipa::path(
    context_path = "/training",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Allenamenti"],
    responses(
        (status = OK, description = "Allenamento eliminata con successo", body = Training, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Allenamento non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("training_id" = i64, Path, description = "ID dell'allenamento"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<training_id>")]
pub fn delete_training_handler(
    key: Result<JWT, ApiError>,
    training_id: i64,
) -> Result<Json<Training>, ApiError> {
    let _key = key?;

    let res = delete_training(training_id)?;
    Ok(Json(res))
}
