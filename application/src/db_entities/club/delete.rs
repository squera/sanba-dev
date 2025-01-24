use diesel::prelude::*;
use domain::{models::full_tables::SportsClub, schema::user_club};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{person_checks::is_administrator, user_checks::is_club_responsible},
};

pub fn authorize_delete_club(
    requesting_user: Claims,
    club_id: String,
) -> Result<SportsClub, ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)?
        || is_club_responsible(requesting_user.subject_id, Some(club_id.clone()), true)?
    {
        is_authorized = true;
    }

    if is_authorized {
        return delete_club(club_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to delete club {}",
                requesting_user.subject_id, club_id,
            ),
        });
    }
}

pub fn delete_club(club_id: String) -> Result<SportsClub, ApiError> {
    use domain::schema::sports_club;

    let connection = &mut establish_connection();

    // TODO effettuare tutti i controlli sui dati collegati alla società sportiva

    let club_to_delete = sports_club::table
        .filter(sports_club::vat_number.eq(&club_id))
        .first::<SportsClub>(connection)?;

    // Eliminazione delle associazioni tra utenti responsabili e società sportiva
    diesel::delete(user_club::table.filter(user_club::club_id.eq(&club_id))).execute(connection)?;

    diesel::delete(sports_club::table.filter(sports_club::vat_number.eq(&club_id)))
        .execute(connection)?;

    Ok(club_to_delete)
}
