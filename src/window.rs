use gst::prelude::ElementExt;
use gst_video::prelude::*;
use gtk::{prelude::*, Application, ApplicationWindow, Button, Orientation};

use crate::gstreamer::GstreamerManager;

pub fn build_ui(app: &Application, _video_path: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mini video player GStreamer + GTK4")
        .default_width(800)
        .default_height(600)
        .build();

    let main_box = gtk::Box::new(Orientation::Vertical, 5);
    let controls_box = gtk::Box::new(Orientation::Horizontal, 5);
    let picture = gtk::Picture::new();
    main_box.append(&picture);

    let play_button = Button::with_label("▶ Play");
    let pause_button = Button::with_label("⏸ Pause");
    let stop_button = Button::with_label("⏹ Stop");

    controls_box.append(&play_button);
    controls_box.append(&pause_button);
    controls_box.append(&stop_button);

    main_box.append(&controls_box);

    window.set_child(Some(&main_box));
    window.present();

    let mut gst_manager = GstreamerManager::new();
    gst_manager.create_pipeline("");

    // BUG: do this to show video screen
    gst_manager.pipeline.set_state(gst::State::Paused).unwrap();
    gst_manager.pipeline.set_state(gst::State::Playing).unwrap();

    let paintable = gst_manager.sink.property::<glib::Object>("paintable");
    picture.set_paintable(Some(&paintable.downcast::<gtk::gdk::Paintable>().unwrap()));

    let pipeline_clone = gst_manager.pipeline.clone();

    // Play
    play_button.connect_clicked(move |_| {
        pipeline_clone
            .set_state(gst::State::Playing)
            .expect("Failed play button");
    });

    let pipeline_clone = gst_manager.pipeline.clone();

    // Pause
    pause_button.connect_clicked(move |_| {
        pipeline_clone
            .set_state(gst::State::Paused)
            .expect("Failed pause button");
    });

    let pipeline_clone = gst_manager.pipeline.clone();

    // Stop
    stop_button.connect_clicked(move |_| {
        pipeline_clone
            .set_state(gst::State::Ready)
            .expect("Failed stop button");
    });

    let pipeline_clone = gst_manager.pipeline.clone();

    // Close
    window.connect_close_request(move |_| {
        pipeline_clone.set_state(gst::State::Null).ok();
        glib::Propagation::Stop
    });
}
