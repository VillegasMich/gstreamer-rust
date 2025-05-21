use gtk::gio;
use gtk::prelude::*;
use gtk::Application;

mod gstreamer;
mod window;

const APP_ID: &str = "org.gtk_rs.gst_video_player";

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
                window::build_ui(app, path.to_str().unwrap());
            }
        }
    });

    app.run();

    Ok(())
}
