use gst::prelude::*;

pub fn run(file_path: &str) {
    gst::init().unwrap();

    let is_network_stream = file_path.starts_with("http://") || file_path.starts_with("https://");

    let pipeline = if is_network_stream {
        gst::parse_launch(&format!(
            "playbin uri={} video-filter=\"videoconvert ! myfilter\" audio-filter=\"audioconvert ! audioresample\"",
            file_path
        ))
        .unwrap()
    } else {
        if !std::path::Path::new(file_path).exists() {
            eprintln!("Error: File does not exist - {}", file_path);
            return;
        }
        gst::parse_launch(&format!(
            "filesrc location={} ! decodebin name=dec dec. ! videoconvert ! autovideosink dec. ! audioconvert ! audioresample ! autoaudiosink",
            file_path
        ))
        .unwrap()
    };

    // ----------------------------------
    // WEBCAM
    // let pipeline = gst::parse_launch("v4l2src ! decodebin ! videoconvert ! autovideosink").unwrap();
    // ----------------------------------

    // Start playing
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
