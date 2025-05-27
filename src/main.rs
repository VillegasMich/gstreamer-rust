use gtk::gio;
use gtk::prelude::*;
use gtk::Application;
use window::WindowManager;

mod file_metadata;
mod filters;
mod gstreamer;
mod window;

const APP_ID: &str = "org.gtk_rs.gst_video_player";
const TITLE: &str = "Mini video player GStreamer + GTK4";
const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const CSS_PATH: &str = "assets/style.css";

fn main() -> Result<(), String> {
    println!("GTK4 Video Player with Gstreamer in Rust!");

    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_open(|app, files, _hint| {
        gst::init().expect("Failed to init GStreamer");
        if let Some(file) = files.first() {
            if let Some(path) = file.path() {
                let window_manager = WindowManager::new(
                    TITLE.to_string(),
                    WIDTH,
                    HEIGHT,
                    path.to_str().expect("Error on file path").to_string(),
                    CSS_PATH.to_string(),
                );
                window_manager.build(app);
            }
        }
    });

    app.run();

    Ok(())
}
