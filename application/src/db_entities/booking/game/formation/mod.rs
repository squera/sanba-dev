use read::get_game_by_formation;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

pub mod create;
pub mod delete;
pub mod read;

fn check_is_formation_of_game(formation_id: i64, game_id: i64) -> Result<(), ApiError> {
    let game = get_game_by_formation(formation_id)?;
    if game.id != game_id {
        return Err(ApiError {
            error_code: 123,
            error_type: ApiErrorType::ApplicationError,
            message: format!(
                "Formation with id {} is not found for game with id {}",
                formation_id, game_id
            ),
            http_status: Status::NotFound,
        });
    } else {
        return Ok(());
    }
}
