use diesel::prelude::*;
use domain::models::others::BookingEvent;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

use crate::{
    authorization::{
        person_checks::is_administrator,
        team_checks::{is_coach_of_team, is_player_of_team, is_responsible_of_team},
        user_checks::is_same_person,
    },
    db_entities::{booking::read::find_booking, recording_session::read::find_recording_session},
};

pub fn can_edit_delete_booking(person_id: i64, booking_id: i64) -> Result<bool, ApiError> {
    use domain::schema::{formation, game};

    let mut is_authorized = false;

    let mut connection = establish_connection();

    let old_booking = find_booking(booking_id)?;

    if is_administrator(person_id)? || is_same_person(person_id, old_booking.booking.author_id) {
        is_authorized = true;
    } else if let Some(event) = &old_booking.event {
        match event {
            BookingEvent::Game(game) => {
                let home_team_id = game::table
                    .filter(game::booking_id.eq(old_booking.booking.id))
                    .inner_join(formation::table.on(game::home_formation_id.eq(formation::id)))
                    .select(formation::team_id)
                    .first::<i64>(&mut connection)?;

                if is_coach_of_team(person_id, Some(home_team_id), true)?
                    || is_responsible_of_team(person_id, home_team_id, true)?
                {
                    is_authorized = true;
                }

                if let Some(visiting_formation_id) = game.visiting_formation_id {
                    let visiting_team_id = formation::table
                        .filter(formation::id.eq(visiting_formation_id))
                        .select(formation::team_id)
                        .first::<i64>(&mut connection)?;

                    if is_coach_of_team(person_id, Some(visiting_team_id), true)?
                        || is_responsible_of_team(person_id, visiting_team_id, true)?
                    {
                        is_authorized = true;
                    }
                }
            }
            BookingEvent::Training(training) => {
                if is_coach_of_team(person_id, Some(training.team_id), true)?
                    || is_responsible_of_team(person_id, training.team_id, true)?
                {
                    is_authorized = true;
                }
            }
        }
    }

    return Ok(is_authorized);
}

pub fn can_read_recording_session(
    person_id: i64,
    recording_session_id: i64,
) -> Result<bool, ApiError> {
    use domain::schema::{formation, game};

    let mut is_authorized = false;

    let mut connection = establish_connection();

    let recording_session = find_recording_session(recording_session_id)?;
    let booking = find_booking(recording_session.recording_session.booking_id)?;

    if is_administrator(person_id)? || is_same_person(person_id, booking.booking.author_id) {
        is_authorized = true;
    } else if let Some(event) = &booking.event {
        match event {
            BookingEvent::Game(game) => {
                let home_team_id = game::table
                    .filter(game::booking_id.eq(booking.booking.id))
                    .inner_join(formation::table.on(game::home_formation_id.eq(formation::id)))
                    .select(formation::team_id)
                    .first::<i64>(&mut connection)?;

                if is_player_of_team(person_id, Some(home_team_id), true)?
                    || is_coach_of_team(person_id, Some(home_team_id), true)?
                    || is_responsible_of_team(person_id, home_team_id, true)?
                {
                    is_authorized = true;
                }

                if let Some(visiting_formation_id) = game.visiting_formation_id {
                    let visiting_team_id = formation::table
                        .filter(formation::id.eq(visiting_formation_id))
                        .select(formation::team_id)
                        .first::<i64>(&mut connection)?;

                    if is_player_of_team(person_id, Some(visiting_team_id), true)?
                        || is_coach_of_team(person_id, Some(visiting_team_id), true)?
                        || is_responsible_of_team(person_id, visiting_team_id, true)?
                    {
                        is_authorized = true;
                    }
                }
            }
            BookingEvent::Training(training) => {
                if is_player_of_team(person_id, Some(training.team_id), true)?
                    || is_coach_of_team(person_id, Some(training.team_id), true)?
                    || is_responsible_of_team(person_id, training.team_id, true)?
                {
                    is_authorized = true;
                }
            }
        }
    }

    return Ok(is_authorized);
}
