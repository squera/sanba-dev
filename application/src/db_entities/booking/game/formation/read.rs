use diesel::prelude::*;
use domain::models::{
    full_tables::{Formation, FormationPlayer, FormationPlayerTag, Game},
    others::FormationPlayerWithTags,
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{
        person_checks::is_administrator,
        team_checks::{is_coach_of_team, is_player_of_team, is_responsible_of_team},
    },
    db_entities::booking::game::formation::check_is_formation_of_game,
};

pub fn authorize_get_formation_player_list(
    requesting_user: Claims,
    game_id: i64,
    formation_id: i64,
) -> Result<Vec<FormationPlayerWithTags>, ApiError> {
    check_is_formation_of_game(formation_id, game_id)?;

    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)? {
        is_authorized = true;
    } else {
        let formation = find_formation(formation_id)?;
        if is_player_of_team(requesting_user.subject_id, Some(formation.team_id), true)?
            || is_coach_of_team(requesting_user.subject_id, Some(formation.team_id), true)?
            || is_responsible_of_team(requesting_user.subject_id, formation.team_id, true)?
        {
            is_authorized = true;
        }
    }

    if is_authorized {
        return get_formation_player_list(formation_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to read formation {}",
                requesting_user.subject_id, formation_id
            ),
        });
    }
}

pub fn get_formation_player_list(
    formation_id: i64,
) -> Result<Vec<FormationPlayerWithTags>, ApiError> {
    use domain::schema::{formation_player, formation_player_tag};

    let connection = &mut establish_connection();

    let mut formation_players_with_tags: Vec<FormationPlayerWithTags> = formation_player::table
        .filter(formation_player::formation_id.eq(&formation_id))
        .select(FormationPlayer::as_select())
        .load(connection)?
        .into_iter()
        .map(|formation_player| FormationPlayerWithTags {
            id: formation_player.id,
            formation_id: formation_player.formation_id,
            player_id: formation_player.player_id,
            rfid_tag_ids: vec![],
        })
        .collect();

    let tags = formation_player_tag::table
        .filter(formation_player_tag::formation_id.eq(&formation_id))
        .load::<FormationPlayerTag>(connection)?;

    populate_rfid_tags(&mut formation_players_with_tags, &tags);

    return Ok(formation_players_with_tags);
}

fn populate_rfid_tags(
    formation_players: &mut Vec<FormationPlayerWithTags>,
    formation_player_tags: &[FormationPlayerTag],
) {
    // Creiamo una mappa da player_id a un vettore di rfid_tag_id per accesso rapido
    let mut tags_map: std::collections::HashMap<i64, Vec<i64>> = std::collections::HashMap::new();

    for tag in formation_player_tags {
        tags_map
            .entry(tag.player_id)
            .or_default()
            .push(tag.rfid_tag_id);
    }

    // Aggiorniamo il campo rfid_tag_ids per ogni FormationPlayerWithTags
    for player in formation_players {
        if let Some(tag_ids) = tags_map.get(&player.player_id) {
            player.rfid_tag_ids = tag_ids.clone();
        } else {
            player.rfid_tag_ids = Vec::new(); // Nessun tag associato
        }
    }
}

pub(crate) fn get_game_by_formation(formation_id: i64) -> Result<Game, ApiError> {
    use domain::schema::game;

    let connection = &mut establish_connection();

    let game = game::table
        .filter(
            game::home_formation_id
                .eq(formation_id)
                .or(game::visiting_formation_id.eq(formation_id)),
        )
        .first::<Game>(connection)?;

    Ok(game)
}

pub(crate) fn find_formation(formation_id: i64) -> Result<Formation, ApiError> {
    use domain::schema::formation;

    let connection = &mut establish_connection();

    let formation = formation::table.find(formation_id).first(connection)?;

    Ok(formation)
}
