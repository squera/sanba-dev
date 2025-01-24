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

// TODO aggiungere un filtro per sport, società sportiva, paginazione
pub fn list_teams() -> Result<Vec<Team>, ApiError> {
    use domain::schema::team;

    let connection = &mut establish_connection();

    let teams = team::table.select(Team::as_select()).load(connection)?;

    return Ok(teams);
}
