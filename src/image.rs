use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};

pub struct ExifInfo {
    pub camera: String,
    pub iso: i32,
    pub aperture: f32,
    pub exposure: f32,
    pub focal_length: f32,
    pub date_time: DateTime<Local>,
}

pub struct MetaInfo {
    pub author: Option<String>,
    pub title: Option<String>,
    pub place: Option<String>,
}

pub struct Image {
    pub source_path: PathBuf,
    pub title: String,
    pub exif_info: Option<ExifInfo>,
    pub meta_info: MetaInfo
}

impl Image {
    pub fn from_source_path(source_path: &Path) -> Self {
        Self {
            source_path: PathBuf::from(source_path),
            title: String::from("Image"),
            exif_info: extract_exif_info(source_path),
            meta_info: MetaInfo {author: None, title: None, place: None}
        }
    }

    pub fn base_name(&self) -> &str {
        self.source_path.file_stem().unwrap().to_str().unwrap()
    }

    pub fn get_date_time(&self) -> DateTime<Local> {
        Local::now()
    }
}

pub fn extract_exif_info(source_path: &Path) -> Option<ExifInfo> {
    None
}


