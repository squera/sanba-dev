use diesel::prelude::*;
use domain::models::full_tables::Person;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

// NB: prima di eliminare la persona, eliminare l'utente associato
pub fn delete_person(person_id: i64) -> Result<Person, ApiError> {
    use domain::schema::person;
    use domain::schema::person::dsl::*;

    let connection = &mut establish_connection();

    let person_to_delete = person
        .filter(id.eq(person_id))
        .first::<Person>(connection)?;

    diesel::delete(person::table.filter(id.eq(person_id))).execute(connection)?;

    Ok(person_to_delete)
}
