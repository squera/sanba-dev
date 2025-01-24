use diesel::prelude::*;
use domain::models::full_tables::Game;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

use crate::db_entities::booking::game::formation::delete::delete_formation;

pub fn delete_game(game_id: i64) -> Result<Game, ApiError> {
    use domain::schema::game;

    let connection = &mut establish_connection();

    let game_to_delete = game::table
        .filter(game::id.eq(&game_id))
        .first::<Game>(connection)?;

    // Eliminazione formazioni
    delete_formation(game_to_delete.home_formation_id)?;

    if let Some(visiting_formation_id) = game_to_delete.visiting_formation_id {
        delete_formation(visiting_formation_id)?;
    }

    // Eliminazione partita
    diesel::delete(game::table.filter(game::id.eq(&game_id))).execute(connection)?;

    Ok(game_to_delete)
}
