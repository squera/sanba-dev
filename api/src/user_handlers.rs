use application::authentication::JWT;
use application::db_entities::user::create::create_user;
use application::db_entities::user::delete::authorize_delete_user;
use application::db_entities::user::login::{extend_token, login_user};
use domain::models::full_tables::User;
use domain::models::others::{LoginRequest, SignupRequest};
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use shared::response_models::ApiError;

/*
/// Restituisce una lista di utenti
///
/// Restituisce tutti gli utenti salvati nel database
#[utoipa::path(
    context_path = "/user",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Utenti"],
    responses(
        (status = 200, description = "Utenti trovati con successo", body = [User], content_type = "application/json")
    )
)]
#[get("/")]
pub fn list_users_handler() -> String {
    let users: Vec<User> = read::list_users();
    serde_json::to_string(&users).unwrap()
}
*/

/*
/// Restituisce un utente
///
/// Restituisce un utente dato il suo ID
#[utoipa::path(
    context_path = "/user",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Utenti"],
    responses(
        (status = 200, description = "Utente trovato con successo", body = User, content_type = "application/json"),
        (status = NOT_FOUND, description = "Utente non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("user_id" = i64, Path, description = "ID dell'utente da cercare"),
    )
)]
#[get("/<user_id>")]
pub fn find_user_handler(user_id: i64) -> Result<String, ApiError> {
    let user = read::list_user(user_id)?;
    Ok(serde_json::to_string(&user).unwrap())
}
*/

/// Permette di eseguire la registrazione
///
/// Dopo i dovuti controlli sui dati, inserisce il nuovo utente nel database. Se viene specificato il codice di invito
/// (e l'email fornita corrisponde a quella dell'invito), l'utente viene collegato alla persona già presente nel database.
/// Altrimenti, viene creata una nuova persona da collegare all'utente.
///
/// ### Chi ha accesso:
/// - Chiunque
#[utoipa::path(
    context_path = "/user",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Utenti"],
    responses(
        (status = CREATED, description = "Utente creato", body = String, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json")
    )
)]
#[post("/signup", data = "<user_data>")]
pub fn signup_handler(user_data: Json<SignupRequest>) -> Result<Json<User>, ApiError> {
    match create_user(user_data.into_inner()) {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(err),
    }
}

/// Permette di eseguire il login
///
/// Se email e password sono corretti, viene restituito il token JWT per poter utilizzare le route che richiedono autenticazione.
///
/// ### Chi ha accesso:
/// - Chiunque
#[utoipa::path(
    context_path = "/user",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Utenti"],
    responses(
        (status = OK, description = "Token di autenticazione", body = String, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nell'autenticazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Utente non trovato", body = ApiError, content_type = "application/json")
    )
)]
#[post("/login", data = "<user_credentials>")]
pub fn login_handler(user_credentials: Json<LoginRequest>) -> Result<Json<String>, ApiError> {
    let token = login_user(user_credentials.into_inner())?;
    Ok(Json(token))
}

/// Permette di prolungare la scadenza del token JWT
///
/// Per non costringere l'utente a rifare il login dopo la scadenza del token JWT anche se sta usando il sistema,
/// si può usare questo endpoint per ottenere un token nuovo con una scadenza prolungata. Se invece l'utente non usa
/// la webapp fino alla scadenza del token, dovrà rifare il login.
///
/// ### Chi ha accesso:
/// - Un utente provvisto di token JWT da rinnovare
#[utoipa::path(
    context_path = "/user",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Utenti"],
    responses(
        (status = OK, description = "Token di autenticazione", body = String, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/refresh-token")]
pub fn refresh_token_handler(key: Result<JWT, ApiError>) -> Result<Json<String>, ApiError> {
    let key = key?;

    let token = extend_token(key.claims.subject_id)?;
    Ok(Json(token))
}

/// Cancella l'utente dal database
///
/// Elimina le informazioni dell'account dell'utente. La persona associata all'utente rimane nel database per permettere a squadre
/// e società di vedere correttamente i dati passati. Se l'utente è attualmente il responsabile di una società, non può essere eliminato.
///
/// ### Chi ha accesso:
/// - L'utente stesso
#[utoipa::path(
    context_path = "/user",      // Path di base che viene aggiunto all'inizio del path specificato nella macro get
    tags = ["Utenti"],
    responses(
        (status = OK, description = "Utente eliminato correttamente", body = User, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Errore nei dati forniti", body = ApiError, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Non è stato fornito un token di autenticazione", body = ApiError, content_type = "application/json"),
        (status = FORBIDDEN, description = "L'utente non è autorizzato a svolgere questa operazione", body = ApiError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Utente non trovato", body = ApiError, content_type = "application/json")
    ),
    params(
        ("user_id" = i64, Path, description = "ID dell'utente da eliminare"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/<user_id>")]
pub fn delete_user_handler(
    key: Result<JWT, ApiError>,
    user_id: i64,
) -> Result<Json<User>, ApiError> {
    let key = key?;

    let res = authorize_delete_user(key.claims, user_id)?;
    Ok(Json(res))
}
