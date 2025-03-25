use std::env::{self};

mod gstreamer;

fn main() -> Result<(), String> {
    println!("Gstreamer in Rust!");
    let mut args = env::args();
    let file_path = args.nth(1);

    match file_path {
        Some(path) => gstreamer::run(&path),
        None => return Err("file argument not found!".to_string()),
    };

    Ok(())
}
