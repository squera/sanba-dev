use diesel::prelude::*;
use domain::models::full_tables::Team;
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{person_checks::is_administrator, team_checks::is_responsible_of_team},
};

pub fn authorize_delete_team(requesting_user: Claims, team_id: i64) -> Result<Team, ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)?
        || is_responsible_of_team(requesting_user.subject_id, team_id, true)?
    {
        is_authorized = true;
    }

    if is_authorized {
        return delete_team(team_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to delete team {}",
                requesting_user.subject_id, team_id,
            ),
        });
    }
}

pub fn delete_team(team_id: i64) -> Result<Team, ApiError> {
    use domain::schema::team;

    let connection = &mut establish_connection();

    // TODO effettuare tutti i controlli sui dati collegati alla squadra

    let team_to_delete = team::table
        .filter(team::id.eq(&team_id))
        .first::<Team>(connection)?;

    diesel::delete(team::table.filter(team::id.eq(&team_id))).execute(connection)?;

    Ok(team_to_delete)
}
