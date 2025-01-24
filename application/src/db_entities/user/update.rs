use diesel::result::Error;
use domain::models::full_tables::User;
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

pub fn update_user(new_user: User) -> Result<User, ApiError> {
    use diesel::prelude::*;
    use domain::schema::user::dsl::*;

    let connection = &mut establish_connection();

    let updated_user: User = match connection.transaction::<_, Error, _>(|connection| {
        diesel::update(user)
            .filter(person_id.eq(new_user.person_id))
            .set(&new_user)
            .execute(connection)?;

        user.find(new_user.person_id).first(connection)
    }) {
        Ok(p) => p,
        Err(err) => {
            return Err(ApiError {
                http_status: Status::InternalServerError,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: format!("Error while updating user - {}", err),
            })
        }
    };

    return Ok(updated_user);
}
