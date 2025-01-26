use diesel::prelude::*;
use domain::models::full_tables::{CoachTeam, PlayerTeam, UserClub};
use infrastructure::establish_connection;
use log::trace;
use shared::response_models::ApiError;

pub fn is_player_of_team(
    person_id: i64,
    team_id: Option<i64>,
    now: bool,
) -> Result<bool, ApiError> {
    use domain::schema::player_team;

    trace!(
        "Checking if user {} is player of team {:?} now: {}",
        person_id,
        team_id,
        now
    );

    let connection = &mut establish_connection();

    let mut query = player_team::table
        .into_boxed()
        .filter(player_team::player_id.eq(person_id));

    if let Some(team_id) = team_id {
        query = query.filter(player_team::team_id.eq(team_id));
    }

    if now {
        query = query.filter(player_team::until_date.is_null());
    }

    match query
        .select(PlayerTeam::as_select())
        .first(connection)
        .optional()?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

/// Verifica se una persona è allenatore di una squadra
///
/// E' possibile fornire uno specifico team_id, altrimenti verrà verificato se l'utente è allenatore di una qualsiasi squadra
/// E' possibile fornire un flag now per verificare se l'utente è allenatore della squadra in questo momento
pub fn is_coach_of_team(person_id: i64, team_id: Option<i64>, now: bool) -> Result<bool, ApiError> {
    use domain::schema::coach_team;

    trace!(
        "Checking if user {} is coach of team {:?} now: {}",
        person_id,
        team_id,
        now
    );

    let connection = &mut establish_connection();

    let mut query = coach_team::table
        .into_boxed()
        .filter(coach_team::coach_id.eq(person_id));

    if let Some(team_id) = team_id {
        query = query.filter(coach_team::team_id.eq(team_id));
    }

    if now {
        query = query.filter(coach_team::until_date.is_null());
    }

    match query
        .select(CoachTeam::as_select())
        .first(connection)
        .optional()?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub fn is_responsible_of_team(user_id: i64, team_id: i64, now: bool) -> Result<bool, ApiError> {
    use domain::schema::{sports_club, team, user_club};

    trace!(
        "Checking if user {} is responsible of the club of team {} now: {}",
        user_id,
        team_id,
        now
    );

    let connection = &mut establish_connection();

    let mut query = sports_club::table
        .into_boxed()
        .inner_join(user_club::table)
        .filter(user_club::user_id.eq(user_id))
        .inner_join(team::table)
        .filter(team::id.eq(team_id));

    if now {
        query = query.filter(user_club::until_date.is_null());
    }

    match query
        .select(UserClub::as_select())
        .first(connection)
        .optional()?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
