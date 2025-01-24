use diesel::prelude::*;
use domain::models::full_tables::UserClub;
use infrastructure::establish_connection;
use log::trace;
use shared::response_models::ApiError;

pub fn is_same_user(user_id_1: i64, user_id_2: i64) -> bool {
    trace!(
        "Checking if user {} is the same as user {}",
        user_id_1,
        user_id_2
    );

    user_id_1 == user_id_2
}

/// Verifica se un utente è responsabile di un club
///
/// E' possibile fornire uno specifico club_id, altrimenti verrà verificato se l'utente è responsabile di un qualsiasi club
/// E' possibile fornire un flag now per verificare se l'utente è responsabile del club in questo momento
pub fn is_club_responsible(
    user_id: i64,
    club_id: Option<String>,
    now: bool,
) -> Result<bool, ApiError> {
    use domain::schema::user_club;

    trace!(
        "Checking if user {} is responsible for club {:?} now: {}",
        user_id,
        club_id,
        now
    );

    let connection = &mut establish_connection();

    let mut query = user_club::table
        .into_boxed()
        .filter(user_club::user_id.eq(user_id));

    if let Some(club_id) = club_id {
        query = query.filter(user_club::club_id.eq(club_id));
    }

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
