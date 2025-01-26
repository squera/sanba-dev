use diesel::prelude::*;
use domain::models::{full_tables::Training, others::TrainingPlayerWithTags};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{
        booking_checks::can_edit_delete_booking,
        person_checks::is_administrator,
        team_checks::{is_coach_of_team, is_responsible_of_team},
    },
    db_entities::booking::training::read::get_training_player_list,
};

use super::read::find_training;

pub fn authorize_delete_training(
    requesting_user: Claims,
    training_id: i64,
) -> Result<Training, ApiError> {
    let training = find_training(training_id)?;
    if can_edit_delete_booking(requesting_user.subject_id, training.booking_id)? {
        return delete_training(training_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to delete training {}",
                requesting_user.subject_id, training_id
            ),
        });
    }
}

pub fn delete_training(training_id: i64) -> Result<Training, ApiError> {
    use domain::schema::{training, training_player, training_player_tag};

    let connection = &mut establish_connection();

    let training_to_delete = training::table
        .filter(training::id.eq(&training_id))
        .first::<Training>(connection)?;

    // Eliminazione delle associazioni tra giocatori e allenamento
    diesel::delete(training_player::table.filter(training_player::training_id.eq(&training_id)))
        .execute(connection)?;

    // Eliminazione delle associazioni tra giocatori, tag RFID e allenamento
    diesel::delete(
        training_player_tag::table.filter(training_player_tag::training_id.eq(&training_id)),
    )
    .execute(connection)?;

    // Eliminazione allenamento
    diesel::delete(training::table.filter(training::id.eq(&training_id))).execute(connection)?;

    Ok(training_to_delete)
}

pub fn authorize_remove_players_from_training(
    requesting_user: Claims,
    training_id: i64,
    player_ids: Vec<i64>,
) -> Result<Vec<TrainingPlayerWithTags>, ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)? {
        is_authorized = true;
    } else {
        let training = find_training(training_id)?;
        if is_coach_of_team(requesting_user.subject_id, Some(training.team_id), true)?
            || is_responsible_of_team(requesting_user.subject_id, training.team_id, true)?
        {
            is_authorized = true;
        }
    }

    if is_authorized {
        return remove_players_from_training(training_id, player_ids);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to remove players from training {}",
                requesting_user.subject_id, training_id
            ),
        });
    }
}

pub fn remove_players_from_training(
    training_id: i64,
    player_ids: Vec<i64>,
) -> Result<Vec<TrainingPlayerWithTags>, ApiError> {
    use domain::schema::{training_player, training_player_tag};

    let connection = &mut establish_connection();

    // Eliminazione delle associazioni tra giocatori e allenamento
    diesel::delete(
        training_player::table
            .filter(training_player::training_id.eq(&training_id))
            .filter(training_player::player_id.eq_any(&player_ids)),
    )
    .execute(connection)?;

    // Eliminazione delle associazioni tra giocatori, tag RFID e allenamento
    diesel::delete(
        training_player_tag::table
            .filter(training_player_tag::training_id.eq(&training_id))
            .filter(training_player_tag::player_id.eq_any(&player_ids)),
    )
    .execute(connection)?;

    let res = get_training_player_list(training_id)?;
    return Ok(res);
}
