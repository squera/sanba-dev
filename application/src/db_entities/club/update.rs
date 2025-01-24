use diesel::prelude::*;
use domain::models::full_tables::{SportsClub, UserClub};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{person_checks::is_administrator, user_checks::is_club_responsible},
    db_entities::club::read::find_club,
    db_entities::user::read::find_user,
};

pub fn authorize_update_club(
    requesting_user: Claims,
    new_club: SportsClub,
) -> Result<SportsClub, ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)?
        || is_club_responsible(
            requesting_user.subject_id,
            Some(new_club.vat_number.clone()),
            true,
        )?
    {
        is_authorized = true;
    }

    if is_authorized {
        return update_club(new_club);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to update club {}",
                requesting_user.subject_id, new_club.vat_number,
            ),
        });
    }
}

pub fn update_club(new_club: SportsClub) -> Result<SportsClub, ApiError> {
    let connection = &mut establish_connection();

    let updated_club = new_club.save_changes::<SportsClub>(connection)?;

    return Ok(updated_club);
}

pub fn authorize_add_club_responsible(
    requesting_user: Claims,
    club_id: String,
    user_id: i64,
) -> Result<(), ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)?
        || is_club_responsible(requesting_user.subject_id, Some(club_id.clone()), true)?
    {
        is_authorized = true;
    }

    if is_authorized {
        return add_club_responsible(club_id, user_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to add a new responsible to club {}",
                requesting_user.subject_id, club_id,
            ),
        });
    }
}

pub fn add_club_responsible(club_id: String, user_id: i64) -> Result<(), ApiError> {
    use domain::schema::user_club;

    let connection = &mut establish_connection();

    let club = find_club(club_id)?;

    let user = find_user(user_id)?;

    diesel::insert_into(user_club::table)
        .values(UserClub {
            user_id: user.person_id,
            club_id: club.vat_number,
            since_date: chrono::Utc::now().naive_utc(),
            until_date: None,
        })
        .execute(connection)?;

    Ok(())
}

pub fn authorize_remove_club_responsible(
    requesting_user: Claims,
    club_id: String,
    user_id: i64,
) -> Result<(), ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)?
        || is_club_responsible(requesting_user.subject_id, Some(club_id.clone()), true)?
    {
        is_authorized = true;
    }

    if is_authorized {
        return remove_club_responsible(club_id, user_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to remove a responsible from club {}",
                requesting_user.subject_id, club_id,
            ),
        });
    }
}

pub fn remove_club_responsible(club_id: String, user_id: i64) -> Result<(), ApiError> {
    use domain::schema::user_club;

    let connection = &mut establish_connection();

    // Verifica che rimanga almeno un responsabile per la società sportiva
    let count = user_club::table
        .filter(user_club::club_id.eq(&club_id))
        .filter(user_club::until_date.is_null())
        .count()
        .get_result::<i64>(connection)?;

    if count <= 1 {
        return Err(ApiError {
            error_code: 123,
            error_type: ApiErrorType::ApplicationError,
            message: "At least one responsible must remain".to_string(),
            http_status: Status::BadRequest,
        });
    }

    let mut user_club = user_club::table
        .filter(user_club::user_id.eq(user_id))
        .filter(user_club::club_id.eq(club_id))
        .get_result::<UserClub>(connection)?;

    user_club.until_date = Some(chrono::Utc::now().naive_utc());

    user_club.save_changes::<UserClub>(connection)?;

    Ok(())
}
