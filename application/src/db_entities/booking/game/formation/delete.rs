use diesel::prelude::*;
use domain::models::{full_tables::Formation, others::FormationPlayerWithTags};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{
        person_checks::is_administrator,
        team_checks::{is_coach_of_team, is_responsible_of_team},
    },
    db_entities::booking::game::formation::{
        check_is_formation_of_game, read::get_formation_player_list,
    },
};

use super::read::find_formation;

pub fn delete_formation(formation_id: i64) -> Result<Formation, ApiError> {
    use domain::schema::{formation, formation_player, formation_player_tag};

    let connection = &mut establish_connection();

    let formation_to_delete = formation::table
        .filter(formation::id.eq(&formation_id))
        .first::<Formation>(connection)?;

    // Eliminazione delle associazioni tra giocatori e formazione
    diesel::delete(
        formation_player::table.filter(formation_player::formation_id.eq(&formation_id)),
    )
    .execute(connection)?;

    // Eliminazione delle associazioni tra giocatori, tag RFID e formazione
    diesel::delete(
        formation_player_tag::table.filter(formation_player_tag::formation_id.eq(&formation_id)),
    )
    .execute(connection)?;

    // Eliminazione formazione
    diesel::delete(formation::table.filter(formation::id.eq(&formation_id))).execute(connection)?;

    Ok(formation_to_delete)
}

pub fn authorize_remove_players_from_formation(
    requesting_user: Claims,
    game_id: i64,
    formation_id: i64,
    player_ids: Vec<i64>,
) -> Result<Vec<FormationPlayerWithTags>, ApiError> {
    check_is_formation_of_game(formation_id, game_id)?;

    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)? {
        is_authorized = true;
    } else {
        let formation = find_formation(formation_id)?;
        if is_coach_of_team(requesting_user.subject_id, Some(formation.team_id), true)?
            || is_responsible_of_team(requesting_user.subject_id, formation.team_id, true)?
        {
            is_authorized = true;
        }
    }

    if is_authorized {
        return remove_players_from_formation(formation_id, player_ids);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to remove players from formation {}",
                requesting_user.subject_id, formation_id
            ),
        });
    }
}

pub fn remove_players_from_formation(
    formation_id: i64,
    player_ids: Vec<i64>,
) -> Result<Vec<FormationPlayerWithTags>, ApiError> {
    use domain::schema::{formation_player, formation_player_tag};

    let connection = &mut establish_connection();

    // Eliminazione delle associazioni tra giocatori e allenamento
    diesel::delete(
        formation_player::table
            .filter(formation_player::formation_id.eq(&formation_id))
            .filter(formation_player::player_id.eq_any(&player_ids)),
    )
    .execute(connection)?;

    // Eliminazione delle associazioni tra giocatori, tag RFID e allenamento
    diesel::delete(
        formation_player_tag::table
            .filter(formation_player_tag::formation_id.eq(&formation_id))
            .filter(formation_player_tag::player_id.eq_any(&player_ids)),
    )
    .execute(connection)?;

    let res = get_formation_player_list(formation_id)?;
    return Ok(res);
}
