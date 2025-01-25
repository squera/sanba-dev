use std::borrow::Cow;

use chrono::{NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

use super::{
    full_tables::{Booking, Person, RecordingSession, SportsClub, Team},
    WithId,
};
use shared::validation::is_future_datetime;

#[derive(Debug, Queryable, Selectable, Insertable, Deserialize, ToSchema, Validate)]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewPerson {
    pub name: String,
    pub surname: String,
}

impl WithId for NewPerson {
    type IdentifiedType = Person;
    type IdType = i64;
    fn to_identified(&self, id: Self::IdType) -> Self::IdentifiedType {
        Person {
            id,
            name: self.name.clone(),
            surname: self.surname.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::player_team)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewPlayerTeam {
    pub player_id: i64,
    pub team_id: i64,
    pub since_date: NaiveDateTime,
    pub until_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::coach_team)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewCoachTeam {
    pub coach_id: i64,
    pub team_id: i64,
    pub since_date: NaiveDateTime,
    pub until_date: Option<NaiveDateTime>,
}

#[derive(
    Debug, Queryable, Selectable, Insertable, AsChangeset, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::sports_club)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewSportsClub {
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
}

impl WithId for NewSportsClub {
    type IdentifiedType = SportsClub;
    type IdType = String;

    fn to_identified(&self, id: Self::IdType) -> Self::IdentifiedType {
        SportsClub {
            vat_number: id,
            name: self.name.clone(),
            address: self.address.clone(),
            city: self.city.clone(),
            phone: self.phone.clone(),
        }
    }
}

#[derive(Debug, Selectable, Insertable, Serialize, Deserialize, ToSchema, Validate)]
#[diesel(table_name = crate::schema::team)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewTeam {
    pub name: String,
    pub club_id: String,
    pub sport: String,
}

impl WithId for NewTeam {
    type IdentifiedType = Team;
    type IdType = i64;
    fn to_identified(&self, id: Self::IdType) -> Self::IdentifiedType {
        Team {
            id,
            name: self.name.clone(),
            club_id: self.club_id.clone(),
            sport: self.sport.clone(),
        }
    }
}

#[derive(Debug, Selectable, Insertable, Serialize, Deserialize, ToSchema, Validate)]
#[diesel(table_name = crate::schema::booking)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[validate(schema(function = "validate_booking"))]
pub struct NewBooking {
    pub author_id: i64,
    #[schema(value_type = String, format = DateTime)]
    #[validate(custom(function = "is_future_datetime"))]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    #[validate(custom(function = "is_future_datetime"))]
    pub end_datetime: NaiveDateTime,
    pub sport: String,
    pub notes: Option<String>,
}

impl WithId for NewBooking {
    type IdentifiedType = Booking;
    type IdType = i64;
    fn to_identified(&self, id: Self::IdType) -> Self::IdentifiedType {
        Booking {
            id,
            author_id: self.author_id,
            start_datetime: self.start_datetime,
            end_datetime: self.end_datetime,
            sport: self.sport.clone(),
            notes: self.notes.clone(),
        }
    }
}

fn validate_booking(data: &NewBooking) -> Result<(), ValidationError> {
    if data.start_datetime >= data.end_datetime {
        Err(
            ValidationError::new("invalid_booking_period").with_message(Cow::Borrowed(
                "The start datetime must be before the end datetime",
            )),
        )
    } else {
        Ok(())
    }
}

#[derive(Debug, Selectable, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::formation)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewFormation {
    pub team_id: i64,
}

#[derive(Debug, Selectable, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::game)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewGame {
    pub home_formation_id: i64,
    pub visiting_formation_id: Option<i64>,
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: Option<NaiveDateTime>,
    pub booking_id: i64,
}

#[derive(Debug, Selectable, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::training)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewTraining {
    pub team_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: Option<NaiveDateTime>,
    pub booking_id: i64,
}

#[derive(Debug, Selectable, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::training_player)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewTrainingPlayer {
    pub training_id: i64,
    pub player_id: i64,
}

#[derive(Debug, Selectable, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::formation_player)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewFormationPlayer {
    pub formation_id: i64,
    pub player_id: i64,
    pub starting: bool,
    #[schema(value_type = String)]
    pub entry_minute: Option<NaiveTime>,
    #[schema(value_type = String)]
    pub exit_minute: Option<NaiveTime>,
}

#[derive(Debug, Selectable, Insertable, Serialize, Deserialize, ToSchema, Validate)]
#[diesel(table_name = crate::schema::recording_session)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[validate(schema(function = "validate_recording_session"))]
pub struct NewRecordingSession {
    pub author_id: i64,
    #[schema(value_type = String, format = DateTime)]
    #[validate(custom(function = "is_future_datetime"))]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    #[validate(custom(function = "is_future_datetime"))]
    pub end_datetime: NaiveDateTime,
    pub booking_id: i64,
}

impl WithId for NewRecordingSession {
    type IdentifiedType = RecordingSession;
    type IdType = i64;
    fn to_identified(&self, id: Self::IdType) -> Self::IdentifiedType {
        RecordingSession {
            id,
            author_id: self.author_id,
            start_datetime: self.start_datetime,
            end_datetime: self.end_datetime,
            booking_id: self.booking_id,
        }
    }
}

fn validate_recording_session(data: &NewRecordingSession) -> Result<(), ValidationError> {
    if data.start_datetime >= data.end_datetime {
        Err(
            ValidationError::new("invalid_recording_session_period").with_message(Cow::Borrowed(
                "The start datetime must be before the end datetime",
            )),
        )
    } else {
        Ok(())
    }
}
