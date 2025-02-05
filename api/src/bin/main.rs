#[macro_use]
extern crate rocket;

use api::{
    booking_handlers, club_handlers, game_handlers, person_handlers, recorded_data_handlers,
    recording_session_handlers, team_handlers, training_handlers, user_handlers,
};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Header;
use rocket::tokio::sync::Mutex;
use rocket::{
    config::LogLevel,
    fairing::AdHoc,
    figment::{
        providers::{Env, Format, Serialized, Toml},
        Figment,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Child;
use std::sync::Arc;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

type StreamMap = Arc<Mutex<HashMap<String, Child>>>;
type Cams = Arc<Mutex<Vec<(String, String)>>>;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Sanbapolis APIs",
    ),
    servers(
        (url = "http://localhost:8000", description = "Server locale"),
        (url = "https://api.example.com", description = "Server remoto")
    ),
    tags(
        (name = "Persone", description = "Operazioni relative alle persone"),
        (name = "Utenti", description = "Operazioni relative agli utenti"),
        (name = "Società sportive", description = "Operazioni relative alle società sportive"),
        (name = "Squadre", description = "Operazioni relative alle squadre"),
        (name = "Prenotazioni", description = "Operazioni relative alle prenotazioni"),
        (name = "Partite", description = "Operazioni relative alle partite e alle formazioni"),
        (name = "Allenamenti", description = "Operazioni relative agli allenamenti"),
        (name = "Sessioni di registrazione", description = "Operazioni relative alle sessioni di registrazione"),
        (name = "Dati registrati", description = "Operazioni relative ai dati registrati dal sistema")
    ),
    paths(
        person_handlers::find_person_handler,
        person_handlers::get_profiles_handler,
        person_handlers::list_people_handler,
        person_handlers::list_people_by_team_handler,
        person_handlers::list_people_by_club_handler,
        person_handlers::create_person_handler,
        person_handlers::new_profile_handler,
        person_handlers::update_person_handler,
        person_handlers::join_team_handler,
        person_handlers::leave_team_handler,
        //person_handlers::delete_person_handler,

        //user_handler::list_users_handler,
        //user_handler::find_user_handler,
        user_handlers::signup_handler,
        user_handlers::login_handler,
        user_handlers::refresh_token_handler,
        user_handlers::delete_user_handler,

        club_handlers::create_club_handler,
        club_handlers::find_club_handler,
        club_handlers::list_clubs_handler,
        club_handlers::update_club_handler,
        club_handlers::add_club_responsible_handler,
        club_handlers::remove_club_responsible_handler,
        club_handlers::delete_club_handler,

        team_handlers::create_team_handler,
        team_handlers::find_team_handler,
        team_handlers::list_teams_handler,
        team_handlers::update_team_handler,
        team_handlers::delete_team_handler,

        booking_handlers::create_booking_handler,
        booking_handlers::find_booking_handler,
        booking_handlers::list_bookings_handler,
        booking_handlers::update_booking_handler,
        booking_handlers::delete_booking_handler,

        game_handlers::find_formation_handler,
        game_handlers::add_players_to_formation_handler,
        game_handlers::delete_players_from_formation_handler,
        game_handlers::delete_game_handler,

        training_handlers::add_training_player_list_handler,
        training_handlers::find_training_player_list_handler,
        training_handlers::delete_training_player_list_handler,
        training_handlers::delete_training_handler,

        recording_session_handlers::create_recording_session_handler,
        recording_session_handlers::find_recording_session_handler,
        recording_session_handlers::list_recording_sessions_by_booking_handler,
        recording_session_handlers::update_recording_session_handler,
        recording_session_handlers::delete_recording_session_handler,

        recorded_data_handlers::list_videos_by_booking_handler,
        recorded_data_handlers::find_video_handler,
        recorded_data_handlers::delete_video_handler,
        recorded_data_handlers::create_screenshot_handler,
        recorded_data_handlers::delete_screenshot_handler,
        recorded_data_handlers::create_timestamp_handler,
        recorded_data_handlers::delete_timestamp_handler,
        recorded_data_handlers::create_clip_handler,
        recorded_data_handlers::share_video_handler,

        recorded_data_handlers::end_streams_capture,
        recorded_data_handlers::init_streams_capture,
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "jwt_token",
            //SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

// rocket configuration
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct RocketConfig {
    address: IpAddr,
    port: u16,
    log_level: LogLevel,
}

impl Default for RocketConfig {
    fn default() -> RocketConfig {
        RocketConfig {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), // change to "0.0.0.0" to open the server to LAN
            port: 8000,
            log_level: LogLevel::Debug,
        }
    }
}

// CORS Fairing
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}

#[launch]
fn rocket() -> _ {
    let streams: StreamMap = Arc::new(Mutex::new(HashMap::new()));

    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error loading .env file: {}", err);
        }
    }

    // To override this configuration (choose one option):
    //  - add a Rocket.toml file in the running directory with the following (change values as needed):
    //      [default]
    //      address = "127.0.0.1"   # Change to "0.0.0.0" to open the server to LAN
    //      port = 8137
    //      log_level = "off"
    //
    //  - run the program with parameters before 'cargo run', such as:
    //      'ROCKET_ADDRESS="0.0.0.0" cargo run'
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(RocketConfig::default()))
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Env::prefixed("ROCKET_").global());

    rocket::custom(figment)
        .attach(AdHoc::config::<RocketConfig>())
        .attach(Cors)
        .manage(streams)
        .mount("/static", FileServer::from("./static"))
        .mount("/dash", FileServer::from("./infrastructure/tmp/dash")) // Serve DASH files
        .mount(
            "/person",
            routes![
                person_handlers::find_person_handler,
                person_handlers::get_profiles_handler,
                person_handlers::list_people_handler,
                person_handlers::list_people_by_team_handler,
                person_handlers::list_people_by_club_handler,
                person_handlers::create_person_handler,
                person_handlers::new_profile_handler,
                person_handlers::update_person_handler,
                person_handlers::join_team_handler,
                person_handlers::leave_team_handler,
                //person_handlers::delete_person_handler
            ],
        )
        .mount(
            "/user",
            routes![
                //user_handler::list_users_handler,
                //user_handler::find_user_handler,
                user_handlers::signup_handler,
                user_handlers::login_handler,
                user_handlers::refresh_token_handler,
                user_handlers::delete_user_handler
            ],
        )
        .mount(
            "/club",
            routes![
                club_handlers::create_club_handler,
                club_handlers::find_club_handler,
                club_handlers::list_clubs_handler,
                club_handlers::update_club_handler,
                club_handlers::add_club_responsible_handler,
                club_handlers::remove_club_responsible_handler,
                club_handlers::delete_club_handler,
            ],
        )
        .mount(
            "/team",
            routes![
                team_handlers::create_team_handler,
                team_handlers::find_team_handler,
                team_handlers::list_teams_handler,
                team_handlers::update_team_handler,
                team_handlers::delete_team_handler,
            ],
        )
        .mount(
            "/booking",
            routes![
                booking_handlers::create_booking_handler,
                booking_handlers::find_booking_handler,
                booking_handlers::list_bookings_handler,
                booking_handlers::update_booking_handler,
                booking_handlers::delete_booking_handler,
            ],
        )
        .mount(
            "/game",
            routes![
                game_handlers::find_formation_handler,
                game_handlers::add_players_to_formation_handler,
                game_handlers::delete_players_from_formation_handler,
                game_handlers::delete_game_handler
            ],
        )
        .mount(
            "/training",
            routes![
                training_handlers::add_training_player_list_handler,
                training_handlers::find_training_player_list_handler,
                training_handlers::delete_training_player_list_handler,
                training_handlers::delete_training_handler
            ],
        )
        .mount(
            "/recording_session",
            routes![
                recording_session_handlers::create_recording_session_handler,
                recording_session_handlers::find_recording_session_handler,
                recording_session_handlers::list_recording_sessions_by_booking_handler,
                recording_session_handlers::update_recording_session_handler,
                recording_session_handlers::delete_recording_session_handler
            ],
        )
        .mount(
            "/video",
            routes![
                recorded_data_handlers::list_videos_by_booking_handler,
                recorded_data_handlers::find_video_handler,
                recorded_data_handlers::delete_video_handler,
                recorded_data_handlers::create_screenshot_handler,
                recorded_data_handlers::delete_screenshot_handler,
                recorded_data_handlers::create_timestamp_handler,
                recorded_data_handlers::delete_timestamp_handler,
                recorded_data_handlers::create_clip_handler,
                recorded_data_handlers::share_video_handler,
            ],
        )
        .mount(
            "/player",
            routes![
                recorded_data_handlers::end_streams_capture,
                recorded_data_handlers::init_streams_capture,
            ],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/openapi.json", ApiDoc::openapi()),
        )
}
