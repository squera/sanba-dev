use std::borrow::Cow;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use shared::validation::{is_future_datetime, is_past_date, is_past_datetime, is_valid_phone};
use utoipa::ToSchema;
use validator::{Validate, ValidationError, ValidationErrors};

use super::{
    full_tables::{Booking, Camera, Game, Person, RecordingSession, Team, Training, User},
    insertions::{NewBooking, NewRecordingSession},
};

#[derive(Serialize, ToSchema)]
pub struct PersonWithUser {
    #[serde(flatten)]
    pub person: Person,
    pub user: Option<User>,
}

#[derive(Serialize, ToSchema)]
pub struct TeamStaff {
    pub team_id: i64,
    pub players: Vec<PersonWithUser>,
    pub coaches: Vec<PersonWithUser>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SignupRequest {
    #[validate(length(equal = 6))]
    pub access_code: Option<String>,
    pub name: String,
    pub surname: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[schema(value_type = String, format = Date)]
    #[validate(custom(function = "is_past_date"))]
    pub birth_date: Option<NaiveDate>,
    pub address: Option<String>,
    pub city: Option<String>,
    #[validate(custom(function = "is_valid_phone"))]
    pub phone: Option<String>,
    // TODO gestire l'immagine del profilo
}

// Se i profili necessitano di parametri, sostituire il bool con una struttura apposita
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct NewProfile {
    pub administrator: Option<bool>,
    #[validate(nested)]
    pub coach: Option<NewCoachProfile>,
    pub fan: Option<bool>,
    pub player: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct NewCoachProfile {
    pub role: String,
    // Aggiungere altri campi se necessario
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[validate(schema(function = "validate_join_info"))]
pub struct JoinInfo {
    /// 0 -> giocatore, 1 -> allenatore
    pub role: u8,
    #[schema(value_type = String, format = Date)]
    #[validate(custom(function = "is_past_datetime"))]
    pub since_date: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    pub until_date: Option<NaiveDateTime>,
}

fn validate_join_info(join_info: &JoinInfo) -> Result<(), ValidationError> {
    if let Some(until_date) = join_info.until_date {
        if join_info.since_date < until_date {
            Ok(())
        } else {
            Err(
                ValidationError::new("invalid_join_info").with_message(Cow::Borrowed(
                    "The since date must be before the until date",
                )),
            )
        }
    } else {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct LeaveInfo {
    /// 0 -> giocatore, 1 -> allenatore
    pub role: u8,
    #[schema(value_type = String, format = Date)]
    pub until_date: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProfileSet {
    pub administrator: Option<bool>,
    pub coach: Option<CoachProfile>,
    pub fan: Option<bool>,
    pub player: Option<PlayerProfile>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlayerProfile {
    pub profiles: Vec<TeamRelation>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CoachProfile {
    pub role: String,
    pub profiles: Vec<TeamRelation>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamRelation {
    #[schema(value_type = String, format = Date)]
    pub since_date: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    pub until_date: Option<NaiveDateTime>,
    pub team: Team,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[validate(schema(function = "validate_booking_data"))]
pub struct NewBookingData {
    #[validate(nested)]
    pub booking: NewBooking,
    #[validate(nested)]
    pub event: Option<NewBookingEvent>, // si può inserire una prenotazione senza partita o allenamento associato
}

fn validate_booking_data(data: &NewBookingData) -> Result<(), ValidationError> {
    todo!()
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum NewBookingEvent {
    Game(GameData),
    Training(TrainingData),
}

// Validate derive is not supported for enums
impl Validate for NewBookingEvent {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let parent_errors = ValidationErrors::new();
        let errors;
        match self {
            NewBookingEvent::Game(game_data) => {
                let game_errors = game_data.validate();

                errors = ValidationErrors::merge(Err(parent_errors), "game", game_errors);
            }
            NewBookingEvent::Training(training_data) => {
                let training_errors = training_data.validate();
                errors = ValidationErrors::merge(Err(parent_errors), "training", training_errors);
            }
        }
        errors
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[validate(schema(function = "validate_game_data"))]
pub struct GameData {
    #[schema(value_type = String, format = DateTime)]
    #[validate(custom(function = "is_future_datetime"))]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    #[validate(custom(function = "is_future_datetime"))]
    pub end_datetime: Option<NaiveDateTime>,
    pub home_team_id: i64,
    pub visiting_team_id: Option<i64>,
}

fn validate_game_data(data: &GameData) -> Result<(), ValidationError> {
    if let Some(end_datetime) = data.end_datetime {
        if data.start_datetime < end_datetime {
            Ok(())
        } else {
            Err(
                ValidationError::new("invalid_game_period").with_message(Cow::Borrowed(
                    "The start datetime must be before the end datetime",
                )),
            )
        }
    } else {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[validate(schema(function = "validate_training_data"))]
pub struct TrainingData {
    #[schema(value_type = String, format = DateTime)]
    #[validate(custom(function = "is_future_datetime"))]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    #[validate(custom(function = "is_future_datetime"))]
    pub end_datetime: Option<NaiveDateTime>,
    pub team_id: i64,
}

fn validate_training_data(data: &TrainingData) -> Result<(), ValidationError> {
    if let Some(end_datetime) = data.end_datetime {
        if data.start_datetime < end_datetime {
            Ok(())
        } else {
            Err(
                ValidationError::new("invalid_training_period").with_message(Cow::Borrowed(
                    "The start datetime must be before the end datetime",
                )),
            )
        }
    } else {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BookingWithEvent {
    pub booking: Booking,
    pub event: Option<BookingEvent>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum BookingEvent {
    Game(Game),
    Training(Training),
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TrainingPlayerTagsData {
    pub training_id: i64,
    pub player_id: i64,
    pub rfid_tag_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TrainingPlayerWithTags {
    pub id: i64,
    pub training_id: i64,
    pub player_id: i64,
    pub rfid_tag_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[validate(schema(function = "validate_fpt_data"))]
pub struct FormationPlayerTagsData {
    pub formation_id: i64,
    pub player_id: i64,
    pub rfid_tag_ids: Vec<i64>,
    pub starting: bool,
    #[schema(value_type = String)]
    pub entry_minute: Option<NaiveTime>,
    #[schema(value_type = String)]
    pub exit_minute: Option<NaiveTime>,
}

fn validate_fpt_data(data: &FormationPlayerTagsData) -> Result<(), ValidationError> {
    match (data.entry_minute, data.exit_minute) {
        (Some(entry_minute), Some(exit_minute)) => {
            if entry_minute < exit_minute {
                Ok(())
            } else {
                Err(
                    ValidationError::new("invalid_fpt_data").with_message(Cow::Borrowed(
                        "The entry minute must be before the exit minute",
                    )),
                )
            }
        }
        _ => Ok(()),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FormationPlayerWithTags {
    pub id: i64,
    pub formation_id: i64,
    pub player_id: i64,
    pub rfid_tag_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RecordingSessionData {
    #[validate(nested)]
    pub recording_session: NewRecordingSession,
    pub camera_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RecordingSessionWithCameras {
    pub recording_session: RecordingSession,
    pub cameras: Vec<Camera>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewScreenshot {
    pub todo: String,
    // TODO aggiungere i campi necessari
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewTimestamp {
    pub todo: String,
    // TODO aggiungere i campi necessari
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewClip {
    pub todo: String,
    // TODO aggiungere i campi necessari
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserList {
    pub todo: String,
    // TODO aggiungere i campi necessari
}
