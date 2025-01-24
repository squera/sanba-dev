use diesel::prelude::*;
use domain::models::full_tables::Team;
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{
        person_checks::is_administrator,
        team_checks::{is_coach_of_team, is_responsible_of_team},
    },
};

pub fn authorize_update_team(requesting_user: Claims, new_team: Team) -> Result<Team, ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)?
        || is_responsible_of_team(requesting_user.subject_id, new_team.id, true)?
        || is_coach_of_team(requesting_user.subject_id, Some(new_team.id), true)?
    {
        is_authorized = true;
    }

    if is_authorized {
        return update_team(new_team);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to update team {}",
                requesting_user.subject_id, new_team.id,
            ),
        });
    }
}

pub fn update_team(new_team: Team) -> Result<Team, ApiError> {
    let connection = &mut establish_connection();

    let updated_team = new_team.save_changes::<Team>(connection)?;

    return Ok(updated_team);
}
