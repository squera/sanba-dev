use application::authentication::JWT;
use application::db_entities::person::create::create_person;
use application::db_entities::person::read;
use application::db_entities::person::read::list_people_by_club;
use application::db_entities::person::update::{
    authorize_add_profile, authorize_join_team, authorize_leave_team, authorize_update_person,
};
use domain::models::full_tables::Person;
use domain::models::insertions::NewPerson;
use domain::models::others::{
    JoinInfo, LeaveInfo, NewProfile, PersonWithUser, ProfileSet, TeamStaff,
};
use domain::models::WithId;
use rocket::serde::json::Json;
use rocket::{get, post, put};
use shared::response_models::ApiError;

/// Restituisce una persona
///
/// Restituisce i dati della persona e dell'utente associato (se esistente).
///
/// ### Chi ha accesso:
/// - Un utente loggato
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Persona trovata con successo", body = PersonWithUser, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Persona non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("person_id" = i64, Path, description = "ID della persona da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/<person_id>")]
pub fn find_person_handler(
    key: Result<JWT, ApiError>,
    person_id: i64,
) -> Result<Json<PersonWithUser>, ApiError> {
    let _key = key?;

    let res = read::find_person(person_id)?;
    Ok(Json(res))
}

/// Restituisce la lista di profili di una persona
///
/// Per ogni profilo (allenatore, giocatore) vengono restituite tutte le relazioni passate e presenti con le squadre.
///
/// ### Chi ha accesso:
/// - Un utente loggato
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Persona trovata con successo", body = ProfileSet, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Persona non trovata", body = ApiError, content_type = "application/json")
    ),
    params(
        ("person_id" = i64, Path, description = "ID della persona da cercare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/<person_id>/profiles", rank = 2)]
pub fn get_profiles_handler(
    key: Result<JWT, ApiError>,
    person_id: i64,
) -> Result<Json<ProfileSet>, ApiError> {
    let _key = key?;

    let res = read::get_profiles(person_id)?;
    Ok(Json(res))
}

/// Restituisce una lista di persone
///
/// Viene restituita la lista di tutti gli utenti con i loro profili
///
/// ### Chi ha accesso:
/// - Un utente loggato
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Persone trovate con successo", body = [PersonWithUser], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/list")]
pub fn list_people_handler(
    key: Result<JWT, ApiError>,
) -> Result<Json<Vec<PersonWithUser>>, ApiError> {
    let _key = key?;

    let res = read::list_people()?;
    Ok(Json(res))
}

/// Restituisce una lista di persone che appartengono a una squadra
///
/// Viene restituita la lista di tutti i membri di una squadra separati per ruolo (allenatori, giocatori)
///
/// ### Chi ha accesso:
/// - Un utente loggato
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Persone trovate con successo", body = TeamStaff, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("team_id" = i64, Path, description = "ID della squadra di cui cercare le persone"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/list-by-team/<team_id>")]
pub fn list_people_by_team_handler(
    key: Result<JWT, ApiError>,
    team_id: i64,
) -> Result<Json<TeamStaff>, ApiError> {
    let _key = key?;

    let res = read::list_people_by_teams(vec![team_id])?
        .into_iter()
        .next()
        .unwrap();
    Ok(Json(res))
}

/// Restituisce una lista di persone che appartengono a una società sportiva
///
/// Viene restituita la lista di tutti i membri di una squadra separati per squadra e ruolo (allenatori, giocatori)
///
/// ### Chi ha accesso:
/// - Un utente loggato
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Persone trovate con successo", body = [TeamStaff], content_type = "application/json"),
        (status = BAD_REQUEST, description = "Token di autenticazione malformato", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("club_id" = String, Path, description = "ID (Partita IVA) della società sportiva di cui cercare le persone"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/list-by-club/<club_id>")]
pub fn list_people_by_club_handler(
    key: Result<JWT, ApiError>,
    club_id: String,
) -> Result<Json<Vec<TeamStaff>>, ApiError> {
    let _key = key?;

    let res = list_people_by_club(club_id)?;
    Ok(Json(res))
}

/// Inserisce una nuova persona
///
/// Viene inserita una nuova persona senza un account associato ad essa. Nel caso venga specificata l'email e la richiesta di
/// invito, viene mandato un invito per email per poter creare un account associato alla persona.
///
/// ### Chi ha accesso:
/// - Chiunque è loggato
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = CREATED, description = "Persona inserita con successo", body = Person, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/", data = "<person>")]
pub fn create_person_handler(
    key: Result<JWT, ApiError>,
    person: Json<NewPerson>,
) -> Result<Json<Person>, ApiError> {
    let _key = key?;

    let res = create_person(person.into_inner())?;
    Ok(Json(res))
    // TODO modificare per fare in modo che il risultato sia con codice 201
}

/// Crea un nuovo profilo per la persona specificata
///
/// Se non è già presente, viene aggiunto il nuovo profilo (allenatore, giocatore, ...) all'utente
///
/// ### Chi ha accesso:
/// - L'utente associato alla persona se presente
/// - Altrimenti chiunque è loggato
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Profilo aggiunto con successo", body = Person, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("person_id" = i64, Path, description = "ID della persona alla quale aggiungere il profilo"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<person_id>/new-profile", data = "<profile_info>")]
pub fn new_profile_handler(
    key: Result<JWT, ApiError>,
    person_id: i64,
    profile_info: Json<NewProfile>,
) -> Result<Json<PersonWithUser>, ApiError> {
    let key = key?;

    let res = authorize_add_profile(key.claims, person_id, profile_info.into_inner())?;
    Ok(Json(res))
}

/// Aggiorna i dati di una persona
///
/// I dati della persona o del suo account vengono aggiornati.
///
/// ### Chi ha accesso:
/// - L'utente associato alla persona se presente
/// - Altrimenti chiunque è loggato
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Dati aggiornati con successo", body = Person, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("person_id" = i64, Path, description = "ID della persona da aggiornare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/<person_id>", data = "<person>")]
pub fn update_person_handler(
    key: Result<JWT, ApiError>,
    person_id: i64,
    person: Json<NewPerson>,
) -> Result<Json<Person>, ApiError> {
    let key = key?;

    let res = authorize_update_person(key.claims, person.into_inner().to_identified(person_id))?;
    Ok(Json(res))
}

/// Registra una persona come parte di una squadra
///
/// Inserisce la relazione tra una persona e una squadra. Questo è possibile solo se la persona ha un profilo adatto alla relazione (giocatore o allenatore).
///
/// ### Chi ha accesso:
/// - L'allenatore della squadra
/// - Il responsabile della società sportiva
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Collegamento inserito con successo", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("person_id" = i64, Path, description = "ID della persona"),
        ("team_id" = i64, Path, description = "ID della squadra a cui collegare la persona"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<person_id>/join/<team_id>", data = "<join_info>")]
pub fn join_team_handler(
    key: Result<JWT, ApiError>,
    person_id: i64,
    team_id: i64,
    join_info: Json<JoinInfo>,
) -> Result<(), ApiError> {
    let key = key?;

    authorize_join_team(key.claims, person_id, team_id, join_info.into_inner())?;
    Ok(())
}

/// Rimuove una persona da una squadra
///
/// Imposta il rapporto persona-squadra come terminato, salvando la data di fine rapporto.
///
/// ### Chi ha accesso:
/// - L'allenatore della squadra
/// - Il responsabile della società sportiva
/// - L'utente associato alla persona se presente
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Collegamento rimosso con successo", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
    ),
    params(
        ("person_id" = i64, Path, description = "ID della persona"),
        ("team_id" = i64, Path, description = "ID della squadra dalla quale rimuovere la persona"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/<person_id>/leave/<team_id>", data = "<leave_info>")]
pub fn leave_team_handler(
    key: Result<JWT, ApiError>,
    person_id: i64,
    team_id: i64,
    leave_info: Json<LeaveInfo>,
) -> Result<(), ApiError> {
    let key = key?;

    authorize_leave_team(key.claims, person_id, team_id, leave_info.into_inner())?;
    Ok(())
}

/*
/// Elimina una persona
///
/// Elimina una persona dato il suo ID
#[utoipa::path(
    context_path = "/person",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Persone"],
    responses(
        (status = OK, description = "Persona eliminata con successo", body = [Person], content_type = "application/json"),
        (status = NOT_FOUND, description = "Persona non trovata")
    ),
    params(
        ("person_id" = u64, Path, description = "ID della persona da eliminare"),
    )
)]
#[delete("/<person_id>")]
pub fn delete_person_handler(person_id: i64) -> Result<String, ApiError> {
    let people = delete::delete_person(person_id)?;
    Ok(serde_json::to_string(&people).unwrap())
}
*/
