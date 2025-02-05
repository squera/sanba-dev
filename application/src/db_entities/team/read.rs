use diesel::prelude::*;
use domain::models::full_tables::Team;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn find_team(team_id: i64) -> Result<Team, ApiError> {
    use domain::schema::team;

    let connection = &mut establish_connection();

    let team = team::table
        .filter(team::id.eq(team_id))
        .select(Team::as_select())
        .get_result(connection)?;

    return Ok(team);
}

pub fn list_teams(
    sport: Option<String>,
    club_id: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Team>, ApiError> {
    use domain::schema::team;

    let connection = &mut establish_connection();

    let mut query = team::table.into_boxed();

    if let Some(sport) = sport {
        query = query.filter(team::sport.eq(sport));
    }

    if let Some(club_id) = club_id {
        query = query.filter(team::club_id.eq(club_id));
    }

    let mut query = query.select(Team::as_select());

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    let teams = query.load(connection)?;

    return Ok(teams);
}
