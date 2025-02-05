use chrono::NaiveDateTime;
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

pub fn list_bookings(
    author_id: Option<i64>,
    from_date: Option<NaiveDateTime>,
    to_date: Option<NaiveDateTime>,
    sport: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<BookingWithEvent>, ApiError> {
    use domain::schema::{booking, game, training};

    let connection = &mut establish_connection();

    let mut query = booking::table.into_boxed();

    if let Some(author_id) = author_id {
        query = query.filter(booking::author_id.eq(author_id));
    }

    if let Some(from_date) = from_date {
        query = query.filter(booking::start_datetime.ge(from_date));
    }

    if let Some(to_date) = to_date {
        query = query.filter(booking::end_datetime.le(to_date));
    }

    if let Some(sport) = sport {
        query = query.filter(booking::sport.eq(sport));
    }

    let mut query = query
        .left_join(game::table)
        .left_join(training::table)
        .select((
            Booking::as_select(),
            Option::<Game>::as_select(),
            Option::<Training>::as_select(),
        ));

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    let bookings_with_events: Vec<(Booking, Option<Game>, Option<Training>)> =
        query.load::<(Booking, Option<Game>, Option<Training>)>(connection)?;

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
