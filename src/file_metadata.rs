use std::fs::metadata;
use std::fs::Metadata;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub metadata: Metadata,
}

impl FileMetadata {
    pub fn new(video_path: &str) -> Self {
        Self {
            path: PathBuf::from(video_path),
            metadata: metadata(video_path).expect("Error extracting file metadata"),
        }
    }
}
