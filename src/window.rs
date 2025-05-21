use std::time::Duration;

use gst::prelude::ElementExt;
use gst_video::prelude::*;
use gtk::{prelude::*, Application, ApplicationWindow, Button, Orientation};

use crate::gstreamer::GstreamerManager;

pub fn build_ui(app: &Application, video_path: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mini video player GStreamer + GTK4")
        .default_width(1280)
        .default_height(720)
        .build();

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_path("assets/style.css");
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let main_box = gtk::Box::new(Orientation::Vertical, 5);
    let slider_box = gtk::Box::new(Orientation::Horizontal, 5);
    let controls_box = gtk::Box::new(Orientation::Horizontal, 5);
    let picture = gtk::Picture::new();
    picture.set_halign(gtk::Align::Center);
    main_box.append(&picture);

    let play_button = Button::with_label("▶ Play");
    play_button.set_valign(gtk::Align::Center);
    let pause_button = Button::with_label("⏸ Pause");
    pause_button.set_valign(gtk::Align::Center);
    let stop_button = Button::with_label("⏹ Stop");
    stop_button.set_valign(gtk::Align::Center);

    controls_box.append(&play_button);
    controls_box.append(&pause_button);
    controls_box.append(&stop_button);

    let progress_slider = gtk::Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    progress_slider.set_hexpand(true);
    progress_slider.set_valign(gtk::Align::Center);

    let playtime_label = gtk::Label::new(None);
    playtime_label.set_halign(gtk::Align::End);
    playtime_label.set_valign(gtk::Align::Center);

    slider_box.append(&playtime_label);
    slider_box.append(&progress_slider);
    main_box.append(&slider_box);

    controls_box.set_halign(gtk::Align::Center);
    main_box.append(&controls_box);

    window.set_child(Some(&main_box));
    window.present();

    let mut gst_manager = GstreamerManager::new();
    gst_manager.create_pipeline(video_path);

    // BUG: do this to show video screen
    gst_manager.pipeline.set_state(gst::State::Paused).unwrap();
    gst_manager.pipeline.set_state(gst::State::Playing).unwrap();

    let paintable = gst_manager.sink.property::<glib::Object>("paintable");
    picture.set_paintable(Some(&paintable.downcast::<gtk::gdk::Paintable>().unwrap()));

    // Slider
    let pipeline_clone = gst_manager.pipeline.clone();
    let progress_slider_clone = progress_slider.clone();

    glib::timeout_add_local(Duration::from_millis(50), move || {
        if let (Some(position), Some(duration)) = (
            pipeline_clone.query_position::<gst::ClockTime>(),
            pipeline_clone.query_duration::<gst::ClockTime>(),
        ) {
            let pos_ns = position.nseconds();
            let dur_ns = duration.nseconds();

            let pos_secs = pos_ns as f64 / 1_000_000_000.0;
            let dur_secs = dur_ns as f64 / 1_000_000_000.0;

            progress_slider_clone.set_range(0.0, dur_secs);
            progress_slider_clone.set_value(pos_secs);
        }

        glib::ControlFlow::Continue
    });

    // Playtime
    let pipeline_clone = gst_manager.pipeline.clone();
    let playtime_label_clone = playtime_label.clone();

    glib::timeout_add_local(Duration::from_millis(50), move || {
        if let (Some(position), Some(duration)) = (
            pipeline_clone.query_position::<gst::ClockTime>(),
            pipeline_clone.query_duration::<gst::ClockTime>(),
        ) {
            let pos_secs = position.seconds();
            let dur_secs = duration.seconds();

            let format_time = |t: u64| format!("{:02}:{:02}", t / 60, t % 60);
            let text = format!("{} / {}", format_time(pos_secs), format_time(dur_secs));

            playtime_label_clone.set_text(&text);
        }

        glib::ControlFlow::Continue
    });

    // Play
    let pipeline_clone = gst_manager.pipeline.clone();

    play_button.connect_clicked(move |_| {
        pipeline_clone
            .set_state(gst::State::Playing)
            .expect("Failed play button");
    });

    // Pause
    let pipeline_clone = gst_manager.pipeline.clone();

    pause_button.connect_clicked(move |_| {
        pipeline_clone
            .set_state(gst::State::Paused)
            .expect("Failed pause button");
    });

    // Stop
    let pipeline_clone = gst_manager.pipeline.clone();

    stop_button.connect_clicked(move |_| {
        pipeline_clone
            .set_state(gst::State::Ready)
            .expect("Failed stop button");
    });

    // BUG: CTRL+q closes the video not the app
    // Close
    let pipeline_clone = gst_manager.pipeline.clone();

    window.connect_close_request(move |_| {
        pipeline_clone.set_state(gst::State::Null).ok();
        glib::Propagation::Stop
    });
}
