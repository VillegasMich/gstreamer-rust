use dirs::home_dir;
use gst::{prelude::*, Element, ElementFactory, Pipeline};

const SRC: &str = "filesrc";
const DECODE: &str = "decodebin";
const CONVERT: &str = "videoconvert";
const SINK: &str = "gtk4paintablesink";

pub struct GstreamerManager {
    pub pipeline: Pipeline,
    pub src: Element,
    pub decode: Element,
    pub convert: Element,
    pub sink: Element,
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
            convert: ElementFactory::make(CONVERT)
                .build()
                .unwrap_or_else(|_| panic!("Could not create {}", CONVERT)),
            sink: ElementFactory::make(SINK)
                .build()
                .unwrap_or_else(|_| panic!("Could not create {}", SINK)),
        }
    }

    pub fn create_pipeline(&mut self, _video_path: &str) {
        let mut video_path = home_dir().unwrap_or_default();
        video_path.push("Videos/Recordings/ZooKeeperEC2.mp4");
        self.src
            .set_property("location", video_path.to_str().unwrap());

        self.pipeline
            .add_many([&self.src, &self.decode, &self.convert, &self.sink])
            .expect("Failed to add elements");

        Element::link_many([&self.src, &self.decode]).expect("Link src → decode failed");
        Element::link_many([&self.convert, &self.sink]).expect("Link convert → sink failed");

        let convert_clone = self.convert.clone();
        self.decode.connect_pad_added(move |_dbin, src_pad| {
            let sink_pad = convert_clone
                .static_pad("sink")
                .expect("Failed to get sink pad from videoconvert");

            if sink_pad.is_linked() {
                return;
            }

            if let Err(err) = src_pad.link(&sink_pad) {
                eprintln!("Pad link failed: {err:?}");
            }
        });
    }
}
