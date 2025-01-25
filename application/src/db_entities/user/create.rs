use diesel::{prelude::*, result::Error};
use domain::models::{
    full_tables::{User, UserInvitation},
    insertions::NewPerson,
    others::SignupRequest,
};
use infrastructure::establish_connection;
use log::trace;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};
use validator::Validate;

use crate::{
    authentication::password::hash_password,
    db_entities::person::{create::create_person, update::update_person},
};

pub fn create_user(request: SignupRequest) -> Result<User, ApiError> {
    request.validate()?;

    // capire se l'access code esiste ed è valido
    //      se è valido, aggiornare i dati della persona e inserire l'utente
    //      se non è valido ma esiste, restituire un errore
    //      se non esiste, creare la nuova persona e inserire l'utente

    use domain::schema::user_invitation;

    trace!("USER SIGNUP");
    trace!("Signup request: {:#?}", request);

    match request.access_code {
        Some(code) => {
            trace!("Access code provided: {}", code);
            let res: Result<Vec<UserInvitation>, Error> = user_invitation::table
                .filter(user_invitation::access_code.eq(code))
                .limit(1)
                .select(UserInvitation::as_select())
                .load(&mut establish_connection());

            match res {
                Ok(mut res) => match res.get_mut(0) {
                    Some(invitation) => {
                        trace!("Access code is valid. Invitation found: {:#?}", invitation);
                        let updated_person = NewPerson {
                            name: request.name,
                            surname: request.surname,
                        };

                        trace!("Updating person: {:#?}", updated_person);
                        update_person(invitation.person_id, updated_person)?;
                        trace!("Person updated successfully.");

                        let new_user = User {
                            person_id: invitation.person_id,
                            // Se l'email dell'invito è vuota, usa quella fornita nella richiesta
                            email: invitation
                                .email
                                .as_ref()
                                .unwrap_or(&request.email)
                                .to_string(),
                            password: hash_password(&request.password).unwrap(),
                            birth_date: request.birth_date,
                            address: request.address,
                            city: request.city,
                            phone: request.phone,
                            profile_image_location: "default.png".to_string(),
                            verified: false,
                            signup_datetime: chrono::Utc::now().naive_utc(),
                        };

                        trace!("Inserting user: {:#?}", new_user);
                        let res = create(new_user)?;
                        trace!("User inserted successfully.");

                        // TODO: eliminare l'invito

                        return Ok(res);
                    }
                    None => {
                        trace!("Access code is invalid.");
                        Err(ApiError {
                            http_status: Status::InternalServerError,
                            error_code: 123,
                            error_type: ApiErrorType::ApplicationError,
                            message: "Access code is invalid".to_string(),
                        })
                    }
                },
                Err(err) => Err(ApiError {
                    http_status: Status::InternalServerError,
                    error_code: 123,
                    error_type: ApiErrorType::ApplicationError,
                    message: format!("Database error - {}", err),
                }),
            }
        }
        None => {
            trace!("No access code provided.");
            let new_person = NewPerson {
                name: request.name,
                surname: request.surname,
            };

            let saved_person = create_person(new_person)?;
            trace!("New person created: {:#?}", saved_person);

            let new_user = User {
                person_id: saved_person.id,
                email: request.email,
                password: hash_password(&request.password).unwrap(),
                birth_date: request.birth_date,
                address: request.address,
                city: request.city,
                phone: request.phone,
                profile_image_location: "default.png".to_string(),
                verified: false,
                signup_datetime: chrono::Utc::now().naive_utc(),
            };

            trace!("Inserting user: {:#?}", new_user);
            let res = create(new_user)?;
            trace!("User inserted successfully.");

            return Ok(res);
        }
    }
}

/// Inserisce un nuovo utente nel database e lo restituisce.
fn create(new_user: User) -> Result<User, ApiError> {
    use domain::schema::user::dsl::*;

    let connection = &mut establish_connection();

    let inserted_user: User = match connection.transaction::<_, Error, _>(|connection| {
        diesel::insert_into(user)
            .values(&new_user)
            .execute(connection)?;

        user.filter(person_id.eq(new_user.person_id))
            .first(connection)
    }) {
        Ok(u) => u,
        Err(err) => {
            return Err(ApiError {
                http_status: Status::InternalServerError,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: format!("Error while inserting new user - {}", err),
            })
        }
    };

    return Ok(inserted_user);
}
