use diesel::prelude::*;
use domain::models::{full_tables::Training, others::TrainingPlayerWithTags};
use infrastructure::establish_connection;
use shared::response_models::ApiError;

use crate::db_entities::booking::training::read::get_training_player_list;

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

pub fn delete_training_players(
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
