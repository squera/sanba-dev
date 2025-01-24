use diesel::prelude::*;
use domain::models::full_tables::Training;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn update_training(new_training: Training) -> Result<Training, ApiError> {
    let connection = &mut establish_connection();

    let updated_training = new_training.save_changes::<Training>(connection)?;

    return Ok(updated_training);
}
