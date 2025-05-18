// use std::env::{self};

use gtk::prelude::*;
use gtk::Application;

mod gstreamer;
mod window;

const APP_ID: &str = "org.gtk_rs.gst_video_player";

fn main() -> Result<(), String> {
    // let mut args = env::args();
    // let video_path = args.nth(1).expect("No video path argument found!");

    println!("GTK4 Video Player with Gstreamer in Rust!");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        gst::init().expect("Failed to init GStreamer");
        window::build_ui(app, "");
    });

    app.run();

    Ok(())
}
