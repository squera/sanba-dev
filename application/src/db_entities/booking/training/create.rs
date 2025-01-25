use diesel::prelude::*;
use diesel::result::Error;
use domain::models::{
    full_tables::{Training, TrainingPlayerTag},
    insertions::{NewTraining, NewTrainingPlayer},
    others::{TrainingPlayerTagsData, TrainingPlayerWithTags},
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};
use validator::Validate;

use crate::db_entities::booking::training::read::get_training_player_list;

pub fn create_training(training: NewTraining) -> Result<Training, ApiError> {
    use domain::schema::training;

    let connection = &mut establish_connection();

    let inserted_training: Training = match connection.transaction::<_, Error, _>(|connection| {
        diesel::insert_into(training::table)
            .values(&training)
            .execute(connection)?;

        // NB: questo metodo per ottenere in ritorno l'allenamento inserito si affida al fatto che gli id siano autoincrementali.
        // Purtroppo attualmente è l'unico modo con MySQL per ottenere l'id dell'allenamento appena inserito.
        // Valutare il passaggio a PostgreSQL per utilizzare il metodo `returning` di Diesel o attendere un supporto a MariaDB.
        training::table.order(training::id.desc()).first(connection)
    }) {
        Ok(p) => p,
        Err(err) => {
            return Err(ApiError {
                http_status: Status::InternalServerError,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: format!("Error while inserting new training - {}", err),
            })
        }
    };

    return Ok(inserted_training);
}

pub fn create_training_player_tags(
    training_id: i64,
    players_data: Vec<TrainingPlayerTagsData>,
) -> Result<Vec<TrainingPlayerWithTags>, ApiError> {
    use domain::schema::{training_player, training_player_tag};

    players_data.validate()?;

    let mut training_players: Vec<NewTrainingPlayer> = vec![];
    let mut training_player_tags: Vec<TrainingPlayerTag> = vec![];

    players_data.iter().for_each(|player_data| {
        training_players.push(NewTrainingPlayer {
            training_id,
            player_id: player_data.player_id,
        });

        player_data.rfid_tag_ids.iter().for_each(|tag_id| {
            training_player_tags.push(TrainingPlayerTag {
                training_id,
                player_id: player_data.player_id,
                rfid_tag_id: *tag_id,
            });
        });
    });

    diesel::insert_into(training_player::table)
        .values(&training_players)
        .execute(&mut establish_connection())?;

    diesel::insert_into(training_player_tag::table)
        .values(&training_player_tags)
        .execute(&mut establish_connection())?;

    let res = get_training_player_list(training_id)?;
    return Ok(res);
}
