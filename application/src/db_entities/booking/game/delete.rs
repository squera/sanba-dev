use diesel::prelude::*;
use domain::models::full_tables::Game;
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims, authorization::booking_checks::can_edit_delete_booking,
    db_entities::booking::game::formation::delete::delete_formation,
};

use super::read::find_game;

pub fn authorize_delete_game(requesting_user: Claims, game_id: i64) -> Result<Game, ApiError> {
    let game = find_game(game_id)?;
    if can_edit_delete_booking(requesting_user.subject_id, game.booking_id)? {
        return delete_game(game_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to delete game {}",
                requesting_user.subject_id, game_id
            ),
        });
    }
}

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
