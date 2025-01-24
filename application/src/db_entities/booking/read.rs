use diesel::prelude::*;
use domain::models::{
    full_tables::{Booking, Game, Training},
    others::{BookingEvent, BookingWithEvent},
};
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn find_booking(booking_id: i64) -> Result<BookingWithEvent, ApiError> {
    use domain::schema::booking;

    let connection = &mut establish_connection();

    let booking = booking::table
        .filter(booking::id.eq(booking_id))
        .select(Booking::as_select())
        .first::<Booking>(connection)?;

    let booking_game = Game::belonging_to(&booking)
        .select(Game::as_select())
        .first::<Game>(connection)
        .optional()?;

    if let Some(game) = booking_game {
        return Ok(BookingWithEvent {
            booking,
            event: Some(BookingEvent::Game(game)),
        });
    } else {
        let booking_training = Training::belonging_to(&booking)
            .select(Training::as_select())
            .first::<Training>(connection)
            .optional()?;

        if let Some(training) = booking_training {
            return Ok(BookingWithEvent {
                booking,
                event: Some(BookingEvent::Training(training)),
            });
        } else {
            return Ok(BookingWithEvent {
                booking,
                event: None,
            });
        }
    }
}

pub fn list_bookings() -> Result<Vec<BookingWithEvent>, ApiError> {
    use domain::schema::{booking, game, training};

    let connection = &mut establish_connection();

    let bookings_with_events: Vec<(Booking, Option<Game>, Option<Training>)> = booking::table
        .left_join(game::table)
        .left_join(training::table)
        .select((
            Booking::as_select(),
            Option::<Game>::as_select(),
            Option::<Training>::as_select(),
        ))
        .load::<(Booking, Option<Game>, Option<Training>)>(connection)?;

    let res = bookings_with_events
        .into_iter()
        .map(|(booking, game, training)| {
            if let Some(game) = game {
                return BookingWithEvent {
                    booking: booking,
                    event: Some(BookingEvent::Game(game)),
                };
            } else if let Some(training) = training {
                return BookingWithEvent {
                    booking: booking,
                    event: Some(BookingEvent::Training(training)),
                };
            } else {
                return BookingWithEvent {
                    booking: booking,
                    event: None,
                };
            }
        })
        .collect();

    return Ok(res);
}
