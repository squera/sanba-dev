use diesel::prelude::*;
use domain::models::{
    full_tables::Booking,
    others::{BookingEvent, BookingWithEvent},
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::booking_checks::can_edit_delete_booking,
    db_entities::booking::{
        game::delete::delete_game, read::find_booking, training::delete::delete_training,
    },
};

pub fn authorize_delete_booking_and_event(
    requesting_user: Claims,
    booking_id: i64,
) -> Result<BookingWithEvent, ApiError> {
    if can_edit_delete_booking(requesting_user.subject_id, booking_id)? {
        return delete_booking_and_event(booking_id);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to delete booking {}",
                requesting_user.subject_id, booking_id
            ),
        });
    }
}

pub fn delete_booking_and_event(booking_id: i64) -> Result<BookingWithEvent, ApiError> {
    // TODO canellare:
    // - sessioni di registrazione

    let booking_to_delete = find_booking(booking_id)?;

    if let Some(event) = &booking_to_delete.event {
        match event {
            BookingEvent::Game(game) => {
                let _ = delete_game(game.id)?;
            }
            BookingEvent::Training(training) => {
                let _ = delete_training(training.id)?;
            }
        }
    }

    delete_booking(booking_id)?;

    return Ok(booking_to_delete);
}

fn delete_booking(booking_id: i64) -> Result<Booking, ApiError> {
    use domain::schema::booking;

    let connection = &mut establish_connection();

    let booking_to_delete = booking::table
        .filter(booking::id.eq(&booking_id))
        .first::<Booking>(connection)?;

    diesel::delete(booking::table.filter(booking::id.eq(&booking_id))).execute(connection)?;

    Ok(booking_to_delete)
}
