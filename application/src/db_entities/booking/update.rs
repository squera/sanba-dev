use diesel::prelude::*;
use domain::models::{
    full_tables::{Booking, Game, Training},
    insertions::NewTraining,
    others::{BookingEvent, BookingWithEvent, NewBookingData, NewBookingEvent},
    WithId,
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::booking_checks::can_edit_delete_booking,
    db_entities::booking::{
        game::{create::create_game_and_formations, update::update_game},
        training::{create::create_training, update::update_training},
    },
};

pub fn authorize_update_booking_and_event(
    requesting_user: Claims,
    booking_id: i64,
    booking_data: NewBookingData,
) -> Result<BookingWithEvent, ApiError> {
    if can_edit_delete_booking(requesting_user.subject_id, booking_id)? {
        return update_booking_and_event(booking_id, booking_data);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to update booking {}",
                requesting_user.subject_id, booking_id
            ),
        });
    }
}

pub fn update_booking_and_event(
    booking_id: i64,
    booking_data: NewBookingData,
) -> Result<BookingWithEvent, ApiError> {
    use domain::schema::{booking, game, training};

    // TODO effettuare controlli sui dati forniti

    let connection = &mut establish_connection();

    let updated_booking = update_booking(booking_data.booking.to_identified(booking_id))?;

    let booking_with_events: (Booking, Option<Game>, Option<Training>) = booking::table
        .left_join(game::table)
        .left_join(training::table)
        .filter(booking::id.eq(updated_booking.id))
        .select((
            Booking::as_select(),
            Option::<Game>::as_select(),
            Option::<Training>::as_select(),
        ))
        .get_result(connection)?;

    let res: BookingWithEvent = match (
        booking_data.event,
        booking_with_events.1,
        booking_with_events.2,
    ) {
        (Some(event), Some(mut game), None) => {
            match event {
                NewBookingEvent::Game(new_game) => {
                    // Aggiorna la partita
                    game.start_datetime = new_game.start_datetime;
                    game.end_datetime = new_game.end_datetime;

                    let updated_game = update_game(game)?;

                    BookingWithEvent {
                        booking: updated_booking,
                        event: Some(BookingEvent::Game(updated_game)),
                    }
                }
                NewBookingEvent::Training(_) => {
                    // Errore: per cambiare evento rimuovere prima l'evento di gioco
                    return Err(ApiError {
                        error_code: 123,
                        error_type: ApiErrorType::ApplicationError,
                        message: "To switch the booking from a game to a training, first delete the game with the appropriate route".to_string(),
                        http_status: Status::BadRequest,
                    });
                }
            }
        }
        (Some(event), None, Some(mut training)) => {
            match event {
                NewBookingEvent::Game(_) => {
                    // Errore: per cambiare evento rimuovere prima l'evento di allenamento
                    return Err(ApiError {
                        error_code: 123,
                        error_type: ApiErrorType::ApplicationError,
                        message: "To switch the booking from a training to a game, first delete the training with the appropriate route".to_string(),
                        http_status: Status::BadRequest,
                    });
                }
                NewBookingEvent::Training(new_training) => {
                    // Aggiorna l' allenamento
                    training.start_datetime = new_training.start_datetime;
                    training.end_datetime = new_training.end_datetime;

                    let updated_training = update_training(training)?;

                    BookingWithEvent {
                        booking: updated_booking,
                        event: Some(BookingEvent::Training(updated_training)),
                    }
                }
            }
        }
        (Some(event), None, None) => {
            // Crea un nuovo evento
            match event {
                NewBookingEvent::Game(new_game) => {
                    // Crea una nuova partita
                    let game = create_game_and_formations(updated_booking.id, new_game)?;

                    BookingWithEvent {
                        booking: updated_booking,
                        event: Some(BookingEvent::Game(game)),
                    }
                }
                NewBookingEvent::Training(new_training) => {
                    // Crea un nuovo allenamento
                    let training = create_training(NewTraining {
                        team_id: new_training.team_id,
                        start_datetime: new_training.start_datetime,
                        end_datetime: new_training.end_datetime,
                        booking_id: updated_booking.id,
                    })?;

                    BookingWithEvent {
                        booking: updated_booking,
                        event: Some(BookingEvent::Training(training)),
                    }
                }
            }
        }
        (None, Some(_), None) => {
            // Errore: per rimuovere un evento, usare la funzione di eliminazione
            return Err(ApiError {
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: "To delete a game, use the appropriate route".to_string(),
                http_status: Status::BadRequest,
            });
        }
        (None, None, Some(_)) => {
            // Errore: per rimuovere un evento, usare la funzione di eliminazione
            return Err(ApiError {
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: "To delete a training, use the appropriate route".to_string(),
                http_status: Status::BadRequest,
            });
        }
        (None, None, None) => {
            // Non fare nulla
            BookingWithEvent {
                booking: updated_booking,
                event: None,
            }
        }
        _ => {
            return Err(ApiError {
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: "Invalid event data".to_string(),
                http_status: Status::InternalServerError,
            });
        }
    };

    return Ok(res);
}

fn update_booking(booking: Booking) -> Result<Booking, ApiError> {
    let connection = &mut establish_connection();

    let updated_booking = booking.save_changes::<Booking>(connection)?;

    return Ok(updated_booking);
}
