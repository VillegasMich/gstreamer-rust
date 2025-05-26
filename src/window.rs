use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

use gst::prelude::ElementExt;
use gst_video::prelude::*;
use gtk::{prelude::*, Application, ApplicationWindow, Button, Orientation};

use crate::{
    filters::{FILTER_NAMES, NO_FILTER},
    gstreamer::GstreamerManager,
};

pub struct WindowManager {
    title: String,
    default_width: i32,
    default_height: i32,
    video_path: String,
    css_path: String,
}

impl WindowManager {
    pub fn new(
        title: String,
        default_width: i32,
        default_height: i32,
        video_path: String,
        css_path: String,
    ) -> Self {
        Self {
            title,
            default_width,
            default_height,
            video_path,
            css_path,
        }
    }

    pub fn build(&self, app: &Application) {
        self.load_css();
        let window = self.build_window(app);
        window.present();
    }

    fn build_window(&self, app: &Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(&self.title)
            .default_width(self.default_width)
            .default_height(self.default_height)
            .build();

        let main_box = gtk::Box::new(Orientation::Vertical, 5);
        let slider_box = gtk::Box::new(Orientation::Horizontal, 5);
        let controls_box = gtk::Box::new(Orientation::Horizontal, 5);
        let filter_selector_box = gtk::Box::new(Orientation::Horizontal, 5);

        let filter_list = gtk::StringList::new(FILTER_NAMES);
        let filter_selector = gtk::DropDown::builder().model(&filter_list).build();
        filter_selector.set_hexpand(true);

        filter_selector_box.append(&filter_selector);

        main_box.append(&filter_selector_box);

        let picture = gtk::Picture::new();
        picture.set_halign(gtk::Align::Center);

        let pause_image = gtk::Image::from_icon_name("media-playback-pause-symbolic");
        pause_image.set_valign(gtk::Align::Center);
        pause_image.set_halign(gtk::Align::Center);
        pause_image.set_pixel_size(64);
        pause_image.set_visible(false);

        let overlay = gtk::Overlay::new();
        overlay.set_child(Some(&picture));
        overlay.add_overlay(&pause_image);

        main_box.append(&overlay);

        let play_button = Button::with_label("▶ Play");
        play_button.set_valign(gtk::Align::Center);
        let pause_button = Button::with_label("⏸  Pause");
        pause_button.set_valign(gtk::Align::Center);
        let stop_button = Button::with_label("⏹  Stop");
        stop_button.set_valign(gtk::Align::Center);
        let volume_toggle = Button::with_label(" ");
        volume_toggle.set_valign(gtk::Align::Center);

        controls_box.append(&play_button);
        controls_box.append(&pause_button);
        controls_box.append(&stop_button);
        controls_box.append(&volume_toggle);

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

        let gst_manager = self.load_gstreamer(picture);

        // Slider
        self.load_slider_movement(progress_slider, &gst_manager);

        // Playtime
        self.load_playtime_indicator(playtime_label, &gst_manager);

        // Play button
        self.load_play_button_logic(pause_image.clone(), play_button, &gst_manager);

        // Pause button
        self.load_pause_button_logic(pause_image.clone(), pause_button, &gst_manager);

        // Stop button
        self.load_stop_button_logic(stop_button, &gst_manager);

        // Volume Toggle
        self.load_volume_button_logic(volume_toggle, &gst_manager);

        // Filter Selector
        self.load_filter_selector_logic(
            filter_selector,
            Rc::new(RefCell::new(gst_manager.clone())),
        );

        // Close
        self.load_close_logic(&window, &gst_manager);

        window
    }

    fn load_css(&self) {
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_path(&self.css_path);
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn load_gstreamer(&self, picture: gtk::Picture) -> GstreamerManager {
        let mut gst_manager = GstreamerManager::new();
        gst_manager.create_pipeline(&self.video_path);

        // Print pipeline properties and elements
        gst_manager.print_pipeline_properties();
        gst_manager.list_elements();

        // BUG: do this to show video screen
        gst_manager.pipeline.set_state(gst::State::Paused).unwrap();
        gst_manager.pipeline.set_state(gst::State::Playing).unwrap();

        let paintable = gst_manager.video_sink.property::<glib::Object>("paintable");
        picture.set_paintable(Some(&paintable.downcast::<gtk::gdk::Paintable>().unwrap()));
        gst_manager
    }

    fn load_slider_movement(&self, progress_slider: gtk::Scale, gst_manager: &GstreamerManager) {
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
    }

    fn load_playtime_indicator(&self, playtime_label: gtk::Label, gst_manager: &GstreamerManager) {
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
    }

    fn load_play_button_logic(
        &self,
        pause_image: gtk::Image,
        play_button: gtk::Button,
        gst_manager: &GstreamerManager,
    ) {
        let pipeline_clone = gst_manager.pipeline.clone();

        play_button.connect_clicked(move |_| {
            pipeline_clone
                .set_state(gst::State::Playing)
                .expect("Failed play button");
            pause_image.set_visible(false);
        });
    }

    fn load_pause_button_logic(
        &self,
        pause_image: gtk::Image,
        pause_button: gtk::Button,
        gst_manager: &GstreamerManager,
    ) {
        let pipeline_clone = gst_manager.pipeline.clone();

        pause_button.connect_clicked(move |_| {
            pipeline_clone
                .set_state(gst::State::Paused)
                .expect("Failed pause button");
            pause_image.set_visible(true);
        });
    }

    fn load_stop_button_logic(&self, stop_button: gtk::Button, gst_manager: &GstreamerManager) {
        let pipeline_clone = gst_manager.pipeline.clone();

        stop_button.connect_clicked(move |_| {
            pipeline_clone
                .set_state(gst::State::Ready)
                .expect("Failed stop button");
        });
    }

    fn load_volume_button_logic(&self, volume_toggle: gtk::Button, gst_manager: &GstreamerManager) {
        // TODO: Insted of toggle add a slider
        let volume_element = gst_manager
            .pipeline
            .clone()
            .by_name("volume0")
            .expect("Volume element not found");

        let is_muted = Rc::new(Cell::new(false));
        let is_muted_clone = is_muted.clone();

        volume_toggle.connect_clicked(move |volume_toggle| {
            let current_mute = is_muted_clone.get();
            let new_volume: f64 = if current_mute { 1.0 } else { 0.0 };
            volume_element.set_property("volume", new_volume);
            volume_toggle.set_label(if current_mute { " " } else { " " });
            is_muted_clone.set(!current_mute);
        });
    }

    fn load_filter_selector_logic(
        &self,
        filter_selector: gtk::DropDown,
        gst_manager: Rc<RefCell<GstreamerManager>>,
    ) {
        let gst_manager_clone = gst_manager.clone();
        filter_selector.connect_selected_item_notify(move |dropdown| {
            if let Some(item) = dropdown.selected_item() {
                if let Some(text) = item
                    .downcast_ref::<gtk::StringObject>()
                    .map(|obj| obj.string().to_owned())
                {
                    if text.eq(NO_FILTER) {
                        println!("filter: '{}'", text);
                        gst_manager_clone
                            .borrow_mut()
                            .remove_filer_and_continue_pipeline();
                    } else {
                        println!("filter: '{}'", text);
                        gst_manager_clone
                            .borrow_mut()
                            .set_filter_and_add_to_pipeline(&text);
                    }
                }
            }
        });
    }

    fn load_close_logic(&self, window: &ApplicationWindow, gst_manager: &GstreamerManager) {
        let pipeline_clone = gst_manager.pipeline.clone();

        window.connect_close_request(move |_| {
            pipeline_clone.set_state(gst::State::Null).ok();
            glib::Propagation::Proceed
        });
    }
}
