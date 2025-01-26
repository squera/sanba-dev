use diesel::{prelude::*, result::Error};
use domain::models::{
    full_tables::{Formation, FormationPlayerTag},
    insertions::{NewFormation, NewFormationPlayer},
    others::{FormationPlayerTagsData, FormationPlayerWithTags},
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};
use validator::Validate;

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

pub fn create_empty_formation(formation: NewFormation) -> Result<Formation, ApiError> {
    use domain::schema::formation;

    let connection = &mut establish_connection();

    let inserted_formation: Formation = match connection.transaction::<_, Error, _>(|connection| {
        diesel::insert_into(formation::table)
            .values(&formation)
            .execute(connection)?;

        // NB: questo metodo per ottenere in ritorno la formazione inserita si affida al fatto che gli id siano autoincrementali.
        // Purtroppo attualmente è l'unico modo con MySQL per ottenere l'id della formazione appena inserita.
        // Valutare il passaggio a PostgreSQL per utilizzare il metodo `returning` di Diesel o attendere un supporto a MariaDB.
        formation::table
            .order(formation::id.desc())
            .first(connection)
    }) {
        Ok(p) => p,
        Err(err) => {
            return Err(ApiError {
                http_status: Status::InternalServerError,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: format!("Error while inserting new formation - {}", err),
            })
        }
    };

    return Ok(inserted_formation);
}

pub fn authorize_add_players_to_formation(
    requesting_user: Claims,
    game_id: i64,
    formation_id: i64,
    players_data: Vec<FormationPlayerTagsData>,
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
        return add_players_to_formation(formation_id, players_data);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to add players to formation {}",
                requesting_user.subject_id, formation_id
            ),
        });
    }
}

pub fn add_players_to_formation(
    formation_id: i64,
    players_data: Vec<FormationPlayerTagsData>,
) -> Result<Vec<FormationPlayerWithTags>, ApiError> {
    use domain::schema::{formation_player, formation_player_tag};

    players_data.validate()?;

    let mut formation_players: Vec<NewFormationPlayer> = vec![];
    let mut formation_player_tags: Vec<FormationPlayerTag> = vec![];

    players_data.iter().for_each(|player_data| {
        formation_players.push(NewFormationPlayer {
            formation_id,
            player_id: player_data.player_id,
            starting: player_data.starting,
            entry_minute: player_data.entry_minute,
            exit_minute: player_data.exit_minute,
        });

        player_data.rfid_tag_ids.iter().for_each(|tag_id| {
            formation_player_tags.push(FormationPlayerTag {
                formation_id,
                player_id: player_data.player_id,
                rfid_tag_id: *tag_id,
            });
        });
    });

    diesel::insert_into(formation_player::table)
        .values(&formation_players)
        .execute(&mut establish_connection())?;

    diesel::insert_into(formation_player_tag::table)
        .values(&formation_player_tags)
        .execute(&mut establish_connection())?;

    let res = get_formation_player_list(formation_id)?;
    return Ok(res);
}
