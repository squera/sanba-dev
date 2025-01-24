use diesel::prelude::*;
use domain::models::full_tables::{Administrator, User};
use infrastructure::establish_connection;
use log::trace;
use shared::response_models::ApiError;

pub fn is_administrator(person_id: i64) -> Result<bool, ApiError> {
    use domain::schema::administrator;

    trace!("Checking if user {} is an administrator", person_id);

    let connection = &mut establish_connection();

    match administrator::table
        .filter(administrator::person_id.eq(person_id))
        .select(Administrator::as_select())
        .first(connection)
        .optional()?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub fn is_person_with_user(person_id: i64) -> Result<bool, ApiError> {
    use domain::schema::user;

    trace!("Checking if person {} is associated to a user", person_id);

    let connection = &mut establish_connection();

    match user::table
        .filter(user::person_id.eq(person_id))
        .select(User::as_select())
        .first(connection)
        .optional()?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
