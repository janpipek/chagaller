use std::path::{Path, PathBuf};

pub struct Image {
    pub source_path: PathBuf,
    pub title: String,
}

impl Image {
    pub fn from_source_path(source_path: &Path) -> Self {
        Self {
            source_path: PathBuf::from(source_path),
            title: String::from("Image"),
        }
    }
}