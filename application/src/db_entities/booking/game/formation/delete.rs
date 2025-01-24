use diesel::prelude::*;
use domain::models::{full_tables::Formation, others::FormationPlayerWithTags};
use infrastructure::establish_connection;
use shared::response_models::ApiError;

use crate::db_entities::booking::game::formation::read::get_formation_player_list;

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

pub fn delete_formation_players(
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
