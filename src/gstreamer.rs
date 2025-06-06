use gst::{prelude::*, Element, ElementFactory, Pipeline};

const SRC: &str = "filesrc";
const DECODE: &str = "decodebin";
const VIDEO_CONVERT: &str = "videoconvert";
const VIDEO_SINK: &str = "gtk4paintablesink";
const AUDIO_CONVERT: &str = "audioconvert";
const AUDIO_SINK: &str = "autoaudiosink";
const VOLUME: &str = "volume";

#[derive(Clone)]
pub struct GstreamerManager {
    pub pipeline: Pipeline,
    pub src: Element,
    pub decode: Element,
    pub vide_convert: Element,
    pub video_sink: Element,
    pub audio_convert: Element,
    pub audio_sink: Element,
    pub volume: Element,
    pub filter: Option<Element>,
}

impl GstreamerManager {
    pub fn new() -> Self {
        Self {
            pipeline: Pipeline::new(),
            src: ElementFactory::make(SRC)
                .build()
                .unwrap_or_else(|_| panic!("Could not create {}", SRC)),
            decode: ElementFactory::make(DECODE)
                .build()
                .unwrap_or_else(|_| panic!("Could not create {}", DECODE)),
            vide_convert: ElementFactory::make(VIDEO_CONVERT)
                .build()
                .unwrap_or_else(|_| panic!("Could not create {}", VIDEO_CONVERT)),
            video_sink: ElementFactory::make(VIDEO_SINK)
                .build()
                .unwrap_or_else(|_| panic!("Could not create {}", VIDEO_SINK)),
            audio_convert: ElementFactory::make(AUDIO_CONVERT)
                .build()
                .unwrap_or_else(|_| panic!("Could not create {}", AUDIO_CONVERT)),
            audio_sink: ElementFactory::make(AUDIO_SINK)
                .build()
                .unwrap_or_else(|_| panic!("Could not create {}", AUDIO_SINK)),
            volume: ElementFactory::make(VOLUME)
                .build()
                .unwrap_or_else(|_| panic!("Could not create {}", VOLUME)),
            filter: None,
        }
    }

    pub fn create_pipeline(&mut self, video_path: &str) {
        self.src.set_property("location", video_path);

        self.pipeline
            .add_many([
                &self.src,
                &self.decode,
                &self.vide_convert,
                &self.video_sink,
                &self.audio_convert,
                &self.audio_sink,
                &self.volume,
            ])
            .expect("Failed to add elements");

        Element::link_many([&self.src, &self.decode]).expect("Link src → decode failed");
        Element::link_many([&self.vide_convert, &self.video_sink])
            .expect("Link video_convert → video_sink failed");
        Element::link_many([&self.audio_convert, &self.volume, &self.audio_sink])
            .expect("Link audio_convert → Link volume → audio_sink failed");

        let video_convert_clone = self.vide_convert.clone();
        let audio_convert_clone = self.audio_convert.clone();
        self.decode.connect_pad_added(move |_dbin, src_pad| {
            let Some(caps) = src_pad.current_caps() else {
                eprintln!("Failed to get caps for pad");
                return;
            };
            let Some(structure) = caps.structure(0) else {
                eprintln!("Failed to get structure for caps");
                return;
            };

            let name = structure.name();

            if name.starts_with("video/") {
                let sink_pad = video_convert_clone
                    .static_pad("sink")
                    .expect("Failed to get sink pad from videoconvert");

                if sink_pad.is_linked() {
                    return;
                }

                if let Err(err) = src_pad.link(&sink_pad) {
                    eprintln!("Pad link failed: {err:?}");
                }
            } else if name.starts_with("audio/") {
                let sink_pad = audio_convert_clone
                    .static_pad("sink")
                    .expect("Failed to get sink pad from audioconvert");

                if sink_pad.is_linked() {
                    return;
                }

                if let Err(err) = src_pad.link(&sink_pad) {
                    eprintln!("Pad link failed: {err:?}");
                }
            }
        });
    }

    pub fn remove_filer_and_continue_pipeline(&mut self) {
        if let Some(old_filter) = self.filter.take() {
            old_filter
                .set_state(gst::State::Null)
                .expect("Failed to set old filter to NULL");

            self.pipeline
                .remove(&old_filter)
                .expect("Failed to remove old filter");
        }
        if let Err(err) = Element::link_many([&self.vide_convert, &self.video_sink]) {
            eprintln!("Failed to relink filter into pipeline: {err:?}");
        } else {
            println!("Filter removed successfully");
            self.filter = None;
        }
    }

    pub fn set_filter_and_add_to_pipeline(&mut self, filter_name: &str) {
        if let Some(old_filter) = self.filter.take() {
            old_filter
                .set_state(gst::State::Null)
                .expect("Failed to set old filter to NULL");

            self.pipeline
                .remove(&old_filter)
                .expect("Failed to remove old filter");
        }

        let new_filter_element = ElementFactory::make(filter_name)
            .build()
            .unwrap_or_else(|_| panic!("Could not create {}", filter_name));

        self.pipeline
            .add(&new_filter_element)
            .expect("Error adding filter to pipeline");
        new_filter_element
            .sync_state_with_parent()
            .expect("Failed to sync filter state");

        self.vide_convert.unlink(&self.video_sink);

        if let Err(err) =
            Element::link_many([&self.vide_convert, &new_filter_element, &self.video_sink])
        {
            eprintln!("Failed to relink filter into pipeline: {err:?}");
        } else {
            println!("Filter '{}' inserted into pipeline", filter_name);
            self.filter = Some(new_filter_element);
        }
    }

    pub fn print_pipeline_properties(&self) {
        println!("\nPipeline properties:");
        for prop in self.pipeline.list_properties() {
            println!("- {}", prop.name());
        }
    }

    pub fn list_elements(&self) {
        println!("\nPipeline elements:");
        for element in self.pipeline.iterate_elements() {
            println!("- {}", element.unwrap().name());
        }
    }
}
