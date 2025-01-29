use std::net::{Ipv4Addr, Ipv6Addr};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::deserialize::{self, FromSqlRow};
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::AsExpression;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Amministratore
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::administrator)]
#[diesel(primary_key(person_id))]
#[diesel(belongs_to(Person, foreign_key = person_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Administrator {
    pub person_id: i64,
}

/// Prenotazione
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::booking)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Booking {
    pub id: i64,
    pub author_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: NaiveDateTime,
    pub sport: String,
    pub notes: Option<String>,
}

/// Videocamera
#[derive(Debug, Clone, Identifiable, Queryable, Selectable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::camera)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Camera {
    pub id: i64,
    #[schema(value_type = String)]
    pub ipv4_address: CustomIpv4Address,
    #[schema(value_type = String)]
    pub ipv6_address: Option<CustomIpv6Address>,
    pub port: u16,
    pub username: String,
    pub password: String,
}

// Implementazione della serializzazione Diesel per gli indirizzi IP
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub struct CustomIpv4Address(Ipv4Addr);

impl ToSql<Text, Mysql> for CustomIpv4Address
where
    String: serialize::ToSql<Text, Mysql>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        let v = self.0.to_string();
        <String as serialize::ToSql<Text, Mysql>>::to_sql(&v, &mut out.reborrow())
    }
}

impl FromSql<Text, Mysql> for CustomIpv4Address
where
    String: deserialize::FromSql<Text, Mysql>,
{
    fn from_sql(bytes: <Mysql as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        <String as deserialize::FromSql<Text, Mysql>>::from_sql(bytes)
            .map(|addr: String| CustomIpv4Address(addr.parse().unwrap()))
    }
}

impl CustomIpv4Address {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub struct CustomIpv6Address(Ipv6Addr);

impl ToSql<Text, Mysql> for CustomIpv6Address
where
    String: serialize::ToSql<Text, Mysql>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        let v = self.0.to_string();
        <String as serialize::ToSql<Text, Mysql>>::to_sql(&v, &mut out.reborrow())
    }
}

impl FromSql<Text, Mysql> for CustomIpv6Address
where
    String: deserialize::FromSql<Text, Mysql>,
{
    fn from_sql(bytes: <Mysql as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        <String as deserialize::FromSql<Text, Mysql>>::from_sql(bytes)
            .map(|addr: String| CustomIpv6Address(addr.parse().unwrap()))
    }
}

/// Relazione tra Camera e RecordingSession
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::camera_session)]
#[diesel(primary_key(session_id, camera_id))]
#[diesel(belongs_to(Camera, foreign_key = camera_id))]
#[diesel(belongs_to(RecordingSession, foreign_key = session_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct CameraSession {
    pub session_id: i64,
    pub camera_id: i64,
}

/// Relazione tra un Video e le sue clip
// NB: Non è possibile usare belongs_to (trait Associations) per due volte sulla stessa tabella, è necessario effettuare dei join manualmente
// (https://github.com/diesel-rs/diesel/issues/2613)
#[derive(Debug, Identifiable, Queryable, Selectable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::clip_video)]
#[diesel(primary_key(original_video_id, clip_id))]
//#[diesel(belongs_to(Video, foreign_key = original_video_id))]     // In conflitto
//#[diesel(belongs_to(Video, foreign_key = clip_id))]               // In conflitto
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ClipVideo {
    pub original_video_id: i64,
    pub clip_id: i64,
}

/// Allenatore
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::coach)]
#[diesel(primary_key(person_id))]
#[diesel(belongs_to(Person, foreign_key = person_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Coach {
    pub person_id: i64,
    pub role: String,
}

/// Relazione tra un allenatore e una squadra
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::coach_team)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Coach, foreign_key = coach_id))]
#[diesel(belongs_to(Team, foreign_key = team_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct CoachTeam {
    pub id: i64,
    pub coach_id: i64,
    pub team_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub since_date: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub until_date: Option<NaiveDateTime>,
}

/// Utente generico (fan)
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::fan)]
#[diesel(primary_key(person_id))]
#[diesel(belongs_to(Person, foreign_key = person_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Fan {
    pub person_id: i64,
}

/// Formazione per giocare una partita
#[derive(
    Debug, Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::formation)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Team, foreign_key = team_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Formation {
    pub id: i64,
    pub team_id: i64,
}

/// Relazione tra Formazione e Giocatore
/// NB: si usa un id come chiave primaria perchè un giocatore può far parte più volte di una formazione
/// (nel caso in cui esca dal gioco e poi rientri)
#[derive(
    Debug, Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::formation_player)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Formation, foreign_key = formation_id))]
#[diesel(belongs_to(Player, foreign_key = player_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct FormationPlayer {
    pub id: i64,
    pub formation_id: i64,
    pub player_id: i64,
    pub starting: bool,
    #[schema(value_type = String)]
    pub entry_minute: Option<NaiveTime>,
    #[schema(value_type = String)]
    pub exit_minute: Option<NaiveTime>,
}

/// Relazione tra Giocaotre e Tag RFID nel contesto di una formazione
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::formation_player_tag)]
#[diesel(primary_key(formation_id, player_id, rfid_tag_id))]
#[diesel(belongs_to(Formation, foreign_key = formation_id))]
#[diesel(belongs_to(Player, foreign_key = player_id))]
#[diesel(belongs_to(RfidTag, foreign_key = rfid_tag_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct FormationPlayerTag {
    pub formation_id: i64,
    pub player_id: i64,
    pub rfid_tag_id: i64,
}

/// Partita
// NB: Non è possibile usare belongs_to per due volte sulla stessa tabella, è necessario effettuare dei join manualmente
// (https://github.com/diesel-rs/diesel/issues/2613)
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::game)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Booking, foreign_key = booking_id))]
//#[diesel(belongs_to(Formation, foreign_key = home_formation_id))]         // In conflitto
//#[diesel(belongs_to(Formation, foreign_key = visiting_formation_id))]     // In conflitto
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Game {
    pub id: i64,
    pub home_formation_id: i64,
    pub visiting_formation_id: Option<i64>,
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: Option<NaiveDateTime>,
    pub booking_id: i64,
}

/// Bucket di InfluxDB
#[derive(
    Debug, Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::influxdb_bucket)]
#[diesel(primary_key(location))]
#[diesel(belongs_to(Team, foreign_key = team_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct InfluxdbBucket {
    pub location: String,
    pub team_id: i64,
    pub name: String,
    pub token: String,
    pub org: String,
    pub db: String,
}

#[derive(
    Debug, Identifiable, Queryable, Selectable, AsChangeset, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::person)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Person {
    /// ID della persona
    pub id: i64,
    /// Nome della persona
    #[schema(examples("Mario", "Luigi"))]
    pub name: String,
    /// Cognome della persona
    #[schema(examples("Rossi", "Bianchi"))]
    pub surname: String,
}

/// Giocatore
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::player)]
#[diesel(primary_key(person_id))]
#[diesel(belongs_to(Person, foreign_key = person_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Player {
    pub person_id: i64,
}

/// Relazione tra Giocatore e Squadra
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::player_team)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Player, foreign_key = player_id))]
#[diesel(belongs_to(Team, foreign_key = team_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct PlayerTeam {
    pub id: i64,
    pub player_id: i64,
    pub team_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub since_date: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub until_date: Option<NaiveDateTime>,
}

/// Sessione di registrazione
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::recording_session)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(belongs_to(Booking, foreign_key = booking_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RecordingSession {
    pub id: i64,
    pub author_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: NaiveDateTime,
    pub booking_id: i64,
}

/// Tag RFID
#[derive(Debug, Identifiable, Queryable, Selectable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::rfid_tag)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RfidTag {
    pub id: i64,
}

/// Screenshot ottenuto da un video
#[derive(
    Debug, Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::screenshot)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Video, foreign_key = video_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Screenshot {
    pub id: i64,
    pub file_location: String,
    pub video_id: i64,
    #[schema(value_type = String)]
    pub instant: NaiveTime,
    pub name: String,
    pub notes: Option<String>,
}

/// Società sportiva
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::sports_club)]
#[diesel(primary_key(vat_number))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SportsClub {
    pub vat_number: String,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
}

/// Squadra
#[derive(
    Debug,
    PartialEq,
    Eq,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::team)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(SportsClub, foreign_key = club_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub club_id: String,
    pub sport: String,
}

/// Segnaposto da inserire in un video
#[derive(
    Debug, Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::time_marker)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Video, foreign_key = video_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TimeMarker {
    pub id: i64,
    #[schema(value_type = String)]
    pub instant: NaiveTime,
    pub video_id: i64,
    pub name: String,
    pub notes: Option<String>,
}

/// Allenamento
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::training)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Booking, foreign_key = booking_id))]
#[diesel(belongs_to(Team, foreign_key = team_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Training {
    pub id: i64,
    pub team_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: Option<NaiveDateTime>,
    pub booking_id: i64,
}

/// Relazione tra un giocatore e un allenamento
#[derive(
    Debug, Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::training_player)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Training, foreign_key = training_id))]
#[diesel(belongs_to(Player, foreign_key = player_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TrainingPlayer {
    pub id: i64,
    pub training_id: i64,
    pub player_id: i64,
}

/// Relazione tra un giocatore e un tag RFID nel contesto di un allenamento
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::training_player_tag)]
#[diesel(primary_key(training_id, player_id, rfid_tag_id))]
#[diesel(belongs_to(Training, foreign_key = training_id))]
#[diesel(belongs_to(Player, foreign_key = player_id))]
#[diesel(belongs_to(RfidTag, foreign_key = rfid_tag_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TrainingPlayerTag {
    pub training_id: i64,
    pub player_id: i64,
    pub rfid_tag_id: i64,
}

/// Descrizione dell'utente
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::user)]
#[diesel(primary_key(person_id))]
#[diesel(belongs_to(Person, foreign_key = person_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(person_id))]
pub struct User {
    pub person_id: i64,
    #[schema(examples("mario@example.com", "luigi@example.com"))]
    pub email: String,
    pub password: String,
    #[schema(value_type = String, format = Date)]
    pub birth_date: Option<NaiveDate>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
    pub profile_image_location: String,
    pub verified: bool,
    #[schema(value_type = String, format = DateTime)]
    pub signup_datetime: NaiveDateTime,
}

/// Relazione tra una società sportiva e gli utenti che ne sono responsabili
#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = crate::schema::user_club)]
#[diesel(primary_key(user_id, club_id, since_date))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(SportsClub, foreign_key = club_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserClub {
    pub user_id: i64,
    pub club_id: String,
    #[schema(value_type = String, format = DateTime)]
    pub since_date: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub until_date: Option<NaiveDateTime>,
}

/// Lista degli inviti ai nuovi utenti
#[derive(
    Debug, Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::user_invitation)]
#[diesel(primary_key(access_code, person_id))]
#[diesel(belongs_to(Person, foreign_key = person_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserInvitation {
    pub access_code: String,
    pub person_id: i64,
    pub email: Option<String>,
}

/// Video
#[derive(
    Debug, Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::video)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(RecordingSession, foreign_key = session_id))]
#[diesel(belongs_to(Camera, foreign_key = camera_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Video {
    pub id: i64,
    pub file_location: String,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub date: NaiveDateTime,
    pub notes: Option<String>,
    pub session_id: i64,
    pub camera_id: i64,
}

/// Permessi di azione di un utente su un video
#[derive(
    Debug, Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize, ToSchema,
)]
#[diesel(table_name = crate::schema::video_user)]
#[diesel(primary_key(user_id, video_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Video, foreign_key = video_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct VideoUser {
    pub user_id: i64,
    pub video_id: i64,
    pub is_owner: bool,
    pub read: bool,
    pub edit: bool,
    pub delete: bool,
    pub share: bool,
}
