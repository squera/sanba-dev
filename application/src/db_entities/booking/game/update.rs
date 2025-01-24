use diesel::prelude::*;
use domain::models::full_tables::Game;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn update_game(new_game: Game) -> Result<Game, ApiError> {
    let connection = &mut establish_connection();

    let updated_game = new_game.save_changes::<Game>(connection)?;

    return Ok(updated_game);
}
