use diesel::{prelude::*, result::Error};
use domain::models::{full_tables::Person, insertions::NewPerson};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};
use validator::Validate;

/// Inserisce una nuova persona nel database e la restituisce.
pub fn create_person(new_person: NewPerson) -> Result<Person, ApiError> {
    use domain::schema::person;

    new_person.validate()?;

    let connection = &mut establish_connection();

    let inserted_person: Person = match connection.transaction::<_, Error, _>(|connection| {
        diesel::insert_into(person::table)
            .values(&new_person)
            .execute(connection)?;

        // NB: questo metodo per ottenere in ritorno la persona inserita si affida al fatto che gli id siano autoincrementali.
        // Purtroppo attualmente è l'unico modo con MySQL per ottenere l'id della persona appena inserita.
        // Valutare il passaggio a PostgreSQL per utilizzare il metodo `returning` di Diesel o attendere un supporto a MariaDB.
        person::table.order(person::id.desc()).first(connection)
    }) {
        Ok(p) => p,
        Err(err) => {
            return Err(ApiError {
                http_status: Status::InternalServerError,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: format!("Error while inserting new person - {}", err),
            })
        }
    };

    return Ok(inserted_person);
}
