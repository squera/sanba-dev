use diesel::prelude::*;
use domain::models::full_tables::Game;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub(crate) fn find_game(game_id: i64) -> Result<Game, ApiError> {
    use domain::schema::game;

    let connection = &mut establish_connection();

    let game = game::table.find(game_id).first(connection)?;

    Ok(game)
}
