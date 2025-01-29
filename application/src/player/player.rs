use std::string::String;
use std::sync::Arc;
use std::collections::HashMap;
use std::process::{Child, Command};
use rocket::tokio::sync::Mutex;
use crate::db_entities;

type StreamMap = Arc<Mutex<HashMap<String, Child>>>;
type Cams = Arc<Mutex<Vec<(String, String)>>>;

// Creation of the pipelines to the stream
pub async fn stream(url: String, name: String, state: &rocket::State<StreamMap>) -> Result<&'static str, String> {
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
        .arg("-remove_at_exit")                     // Remove files when process ends
        .arg("1")
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

// Init for the cameras info
pub async fn list_cameras() -> Result<Cams, &'static str> {
    let cams:Cams=Arc::new(Mutex::new(Vec::new()));
    let db_list=db_entities::camera::read::list_cameras().unwrap();

    if db_list.is_empty() {
        return Err("Couldn't load from database");
    }

    for (index, cam) in db_list.iter().enumerate() {
        let url=format!("rtsp://{}:{}@{}:{}", cam.username, cam.password, cam.ipv4_address.to_string(), cam.port.to_string());
        let name=format!("CAM{}", index);

        let mut cams_lock = cams.lock().await;
        cams_lock.push((name, url));
    }

    Ok(cams)
}