use diesel::{prelude::*, result::Error};
use domain::models::{
    full_tables::Game,
    insertions::{NewFormation, NewGame},
    others::GameData,
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use super::formation::create::create_empty_formation;

pub fn create_game_and_formations(booking_id: i64, game_data: GameData) -> Result<Game, ApiError> {
    let home_formation_id = create_empty_formation(NewFormation {
        team_id: game_data.home_team_id,
    })?
    .id;

    let visiting_formation_id = if let Some(visiting_team_id) = game_data.visiting_team_id {
        Some(
            create_empty_formation(NewFormation {
                team_id: visiting_team_id,
            })?
            .id,
        )
    } else {
        None
    };

    let game = create_game(NewGame {
        home_formation_id: home_formation_id,
        visiting_formation_id: visiting_formation_id,
        start_datetime: game_data.start_datetime,
        end_datetime: game_data.end_datetime,
        booking_id: booking_id,
    })?;

    return Ok(game);
}

fn create_game(game: NewGame) -> Result<Game, ApiError> {
    use domain::schema::game;

    let connection = &mut establish_connection();

    let inserted_game: Game = match connection.transaction::<_, Error, _>(|connection| {
        diesel::insert_into(game::table)
            .values(&game)
            .execute(connection)?;

        // NB: questo metodo per ottenere in ritorno la partita inserita si affida al fatto che gli id siano autoincrementali.
        // Purtroppo attualmente è l'unico modo con MySQL per ottenere l'id della partita appena inserita.
        // Valutare il passaggio a PostgreSQL per utilizzare il metodo `returning` di Diesel o attendere un supporto a MariaDB.
        game::table.order(game::id.desc()).first(connection)
    }) {
        Ok(p) => p,
        Err(err) => {
            return Err(ApiError {
                http_status: Status::InternalServerError,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: format!("Error while inserting new game - {}", err),
            })
        }
    };

    return Ok(inserted_game);
}
