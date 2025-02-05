use diesel::prelude::*;
use domain::models::full_tables::SportsClub;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn find_club(club_id: String) -> Result<SportsClub, ApiError> {
    use domain::schema::sports_club;

    let connection = &mut establish_connection();

    let club = sports_club::table
        .filter(sports_club::vat_number.eq(club_id))
        .select(SportsClub::as_select())
        .get_result(connection)?;

    return Ok(club);
}

pub fn list_clubs(limit: Option<i64>, offset: Option<i64>) -> Result<Vec<SportsClub>, ApiError> {
    use domain::schema::sports_club;

    let connection = &mut establish_connection();

    let mut query = sports_club::table
        .select(SportsClub::as_select())
        .into_boxed();

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    let clubs = query.load(connection)?;

    return Ok(clubs);
}
