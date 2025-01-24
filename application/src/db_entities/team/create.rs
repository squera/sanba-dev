use diesel::{prelude::*, result::Error};
use domain::models::{full_tables::Team, insertions::NewTeam};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{person_checks::is_administrator, user_checks::is_club_responsible},
};

pub fn authorize_create_team(requesting_user: Claims, new_team: NewTeam) -> Result<Team, ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)? {
        is_authorized = true;
    } else if is_club_responsible(
        requesting_user.subject_id,
        Some(new_team.club_id.clone()),
        true,
    )? {
        is_authorized = true;
    }

    if is_authorized {
        return create_team(new_team);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to create a new team",
                requesting_user.subject_id,
            ),
        });
    }
}

/// Inserisce una nuova squadra nel database e la restituisce.
pub fn create_team(new_team: NewTeam) -> Result<Team, ApiError> {
    use domain::schema::team;

    let connection = &mut establish_connection();

    let inserted_team: Team = match connection.transaction::<_, Error, _>(|connection| {
        diesel::insert_into(team::table)
            .values(&new_team)
            .execute(connection)?;

        // NB: questo metodo per ottenere in ritorno la squadra inserita si affida al fatto che gli id siano autoincrementali.
        // Purtroppo attualmente è l'unico modo con MySQL per ottenere l'id della squadra appena inserita.
        // Valutare il passaggio a PostgreSQL per utilizzare il metodo `returning` di Diesel o attendere un supporto a MariaDB.
        team::table.order(team::id.desc()).first(connection)
    }) {
        Ok(p) => p,
        Err(err) => {
            return Err(ApiError {
                http_status: Status::InternalServerError,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: format!("Error while inserting new team - {}", err),
            })
        }
    };

    return Ok(inserted_team);
}
