#[macro_use]
extern crate rocket;

use std::string::String;
use std::path::Path;
use std::sync::Arc;
use std::collections::HashMap;
use std::process::{Child, Command};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::fs::{NamedFile, FileServer};
use tokio::sync::Mutex;

// Shared variables
type StreamMap = Arc<Mutex<HashMap<String, Child>>>;
type Cams = Arc<Vec<(String, String)>>;

// Request to boot the streams reading
#[get("/start")]
async fn boot(state: &rocket::State<StreamMap>, cams: &rocket::State<Cams>)
{
    println!("Boot endpoint triggered!"); // Debug log
    for (name, url) in cams.iter()
    {
        println!("Attempting to start stream: {}, URL: {}", name, url); // Debug log for each stream
        if let Err(e) = stream(url.clone(), name.clone(), state).await
        {
            eprintln!("Error starting stream {}: {}", name, e);
        }
        else
        {
            println!("Started stream for: {}", url);
        }
    }
}

// Creation of the pipelines to the stream
async fn stream(url: String, name: String, state: &rocket::State<StreamMap>) -> Result<&'static str, String>
{
    let dash_path=format!("./tmp/dash/{}", &name);

    // Create DASH output directory
    if let Err(e) = std::fs::create_dir_all(&dash_path)
    {
        return Err(format!("Failed to create directory {}: {}", dash_path, e));
    }

    // Start the FFmpeg process
    let ffmpeg_command = Command::new("ffmpeg")
        .arg("-i")                                  // Input
        .arg(&url)                                  // Input RTSP URL
        .arg("-f")                                  // Output
        .arg("dash")                                // DASH output format
        .arg(format!("{}/manifest.mpd", dash_path)) // DASH output location
        .spawn();

    // Save the FFmpeg process in the shared state + error management
    match ffmpeg_command {
        Ok(child) => {
            let mut streams = state.lock().await;
            streams.insert(url.clone(), child);
            Ok("Stream started")
        }
        Err(e) => Err(format!("Failed to start FFmpeg: {}", e)),
    }
}

// Request to stop all pipelines
#[get("/stop")]
async fn stop_stream(state: &rocket::State<StreamMap>, cams: &rocket::State<Cams>) -> &'static str
{
    let mut streams = state.lock().await;

    for (_name, url) in cams.iter()
    {
        if let Some(mut child) = streams.remove(url)
        {
            if let Err(e) = child.kill()
            {
                eprintln!("Failed to stop FFmpeg process: {}", e);
            }
            println!("Stream stopped");
        }
        else
        {
            println!("Stream not found");
        }
    }
    "Stream stopped"
}

// Request handler to serve the index page
#[get("/")]
async fn serve_index() -> Option<NamedFile>
{
    NamedFile::open(Path::new("src/static/index.html")).await.ok()
}

// CORS Fairing
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors
{
    fn info(&self) -> Info
    {
        Info
        {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r rocket::Request<'_>, response: &mut rocket::Response<'r>)
    {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}

// Main function
#[launch]
fn rocket() -> _
{
    let cams=init_cams();

    let streams: StreamMap = Arc::new(Mutex::new(HashMap::new()));

    rocket::build()
        .manage(streams) // Manage streams state as shared variable
        .manage(cams) // Manage cameras urls and name as shared variable
        .mount("/dash", FileServer::from("./tmp/dash")) // Serve DASH files
        .mount("/", routes![boot, stop_stream, serve_index]) // Mount the routes
        .attach(Cors) // Attach CORS
}

// Init for the cameras info
fn init_cams() -> Cams
{
    Arc::new(vec!
    [
        ("CAM1".to_string(), "rtsp://192.168.56.1:8554/stream".to_string()),
        ("CAM2".to_string(), "rtsp://192.168.56.1:8555/stream".to_string())
    ])
}