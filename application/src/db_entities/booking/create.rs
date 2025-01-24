use diesel::{prelude::*, result::Error};
use domain::models::{
    full_tables::Booking,
    insertions::{NewBooking, NewTraining},
    others::{BookingEvent, BookingWithEvent, NewBookingData, NewBookingEvent},
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{
        person_checks::is_administrator,
        team_checks::{is_coach_of_team, is_responsible_of_team},
    },
};

use super::{game::create::create_game_and_formations, training::create::create_training};

pub fn authorize_create_booking(
    requesting_user: Claims,
    new_booking_data: NewBookingData,
) -> Result<BookingWithEvent, ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)? {
        is_authorized = true;
    } else if let Some(event) = &new_booking_data.event {
        match event {
            NewBookingEvent::Game(new_game) => {
                if is_coach_of_team(
                    requesting_user.subject_id,
                    Some(new_game.home_team_id),
                    true,
                )? || is_responsible_of_team(
                    requesting_user.subject_id,
                    new_game.home_team_id,
                    true,
                )? {
                    is_authorized = true;
                }

                if let Some(visiting_team_id) = new_game.visiting_team_id {
                    if is_coach_of_team(requesting_user.subject_id, Some(visiting_team_id), true)?
                        || is_responsible_of_team(
                            requesting_user.subject_id,
                            visiting_team_id,
                            true,
                        )?
                    {
                        is_authorized = true;
                    }
                }
            }
            NewBookingEvent::Training(new_training) => {
                if is_coach_of_team(requesting_user.subject_id, Some(new_training.team_id), true)?
                    || is_responsible_of_team(
                        requesting_user.subject_id,
                        new_training.team_id,
                        true,
                    )?
                {
                    is_authorized = true;
                }
            }
        }
    }

    if is_authorized {
        return create_booking(new_booking_data);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to create a new booking",
                requesting_user.subject_id,
            ),
        });
    }
}

/// Inserisce una nuova prenotazione nel database e la restituisce.
pub fn create_booking(new_booking_data: NewBookingData) -> Result<BookingWithEvent, ApiError> {
    let booking = create_empty_booking(new_booking_data.booking)?;

    if let Some(event) = new_booking_data.event {
        match event {
            NewBookingEvent::Game(new_game) => {
                let game = create_game_and_formations(booking.id, new_game)?;

                return Ok(BookingWithEvent {
                    booking: booking,
                    event: Some(BookingEvent::Game(game)),
                });
            }
            NewBookingEvent::Training(new_training) => {
                let training = create_training(NewTraining {
                    team_id: new_training.team_id,
                    start_datetime: new_training.start_datetime,
                    end_datetime: new_training.end_datetime,
                    booking_id: booking.id,
                })?;

                return Ok(BookingWithEvent {
                    booking: booking,
                    event: Some(BookingEvent::Training(training)),
                });
            }
        }
    }

    return Ok(BookingWithEvent {
        booking: booking,
        event: None,
    });
}

fn create_empty_booking(new_booking: NewBooking) -> Result<Booking, ApiError> {
    use domain::schema::booking;

    let connection = &mut establish_connection();

    let inserted_booking: Booking = match connection.transaction::<_, Error, _>(|connection| {
        diesel::insert_into(booking::table)
            .values(&new_booking)
            .execute(connection)?;

        // NB: questo metodo per ottenere in ritorno la prenotazione inserita si affida al fatto che gli id siano autoincrementali.
        // Purtroppo attualmente è l'unico modo con MySQL per ottenere l'id della prenotazione appena inserita.
        // Valutare il passaggio a PostgreSQL per utilizzare il metodo `returning` di Diesel o attendere un supporto a MariaDB.
        booking::table.order(booking::id.desc()).first(connection)
    }) {
        Ok(p) => p,
        Err(err) => {
            return Err(ApiError {
                http_status: Status::InternalServerError,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: format!("Error while inserting new booking - {}", err),
            })
        }
    };

    return Ok(inserted_booking);
}

// fn create_formation(formation: FormationWithPlayers) -> Result<i64, ApiError> {
//     use domain::schema::formation;

//     let connection = &mut establish_connection();

//     // Inserimento formazione
//     let new_formation = NewFormation {
//         team_id: formation.team_id,
//     };

//     let inserted_formation: Formation = match connection.transaction::<_, Error, _>(|connection| {
//         diesel::insert_into(formation::table)
//             .values(&new_formation)
//             .execute(connection)?;

//         // NB: questo metodo per ottenere in ritorno la formazione inserita si affida al fatto che gli id siano autoincrementali.
//         // Purtroppo attualmente è l'unico modo con MySQL per ottenere l'id della formazione appena inserita.
//         // Valutare il passaggio a PostgreSQL per utilizzare il metodo `returning` di Diesel o attendere un supporto a MariaDB.
//         formation::table
//             .order(formation::id.desc())
//             .first(connection)
//     }) {
//         Ok(p) => p,
//         Err(err) => {
//             return Err(ApiError {
//                 http_status: Status::InternalServerError,
//                 error_code: 123,
//                 error_type: None,
//                 message: format!("Error while inserting new formation - {}", err),
//             })
//         }
//     };

//     // Inserimento giocatori nella formazione
//     let new_formation_players: Vec<NewFormationPlayer> = formation
//         .players
//         .iter()
//         .map(|player| NewFormationPlayer {
//             formation_id: inserted_formation.id,
//             player_id: player.player_id,
//             starting: player.starting,
//             entry_minute: player.entry_minute,
//             exit_minute: player.exit_minute,
//         })
//         .collect();

//     diesel::insert_into(formation_player::table)
//         .values(&new_formation_players)
//         .execute(connection)?;

//     return Ok(inserted_formation.id);
// }
