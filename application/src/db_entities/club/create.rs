use diesel::prelude::*;
use domain::models::full_tables::SportsClub;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

use crate::db_entities::club::update::add_club_responsible;

/// Inserisce una nuova società sportiva nel database e la restituisce.
pub fn create_club(new_club: SportsClub, user_id: i64) -> Result<SportsClub, ApiError> {
    use domain::schema::sports_club;

    let connection = &mut establish_connection();

    diesel::insert_into(sports_club::table)
        .values(&new_club)
        .execute(connection)?;

    add_club_responsible(new_club.vat_number.clone(), user_id)?;

    return Ok(new_club);
}
