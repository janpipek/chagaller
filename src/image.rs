use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use exif;
use exif::{Exif, Tag};
use std::io::BufReader;

pub struct ExifInfo {
    pub camera: String,
    pub iso: String,
    pub aperture: String,
    pub exposure: String,
    pub focal_length: String,
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
        match &self.exif_info {
            Some(exif_info) => {
                exif_info.date_time
            },
            None => Local::now()
        }
    }
}

pub fn extract_exif_info(source_path: &Path) -> Option<ExifInfo> {
    let file = std::fs::File::open(source_path).unwrap();
    let mut reader = BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    match exif_reader.read_from_container(&mut reader) {
        Ok(exif_source) => {
            Some(ExifInfo {
                camera: extract_tag(&exif_source, Tag::Model, None),
                iso: extract_tag(&exif_source, Tag::ISOSpeed, None),
                aperture: extract_tag(&exif_source, Tag::ApertureValue, Some(|x| format!("f/{}", x))),
                exposure: extract_tag(&exif_source, Tag::ShutterSpeedValue, None),
                focal_length: extract_tag(&exif_source, Tag::FocalLength, None),
                date_time: Local::now(),
            })
        }
        Err(_) => None
    }
}

fn extract_tag(exif_source: &Exif, tag: Tag, formatter: Option<fn(String) -> String>) -> String {
    let field = exif_source.get_field(tag, exif::In::PRIMARY);
    match field {
        None => String::from("Unknown"),
        Some(f) => {
            let v = f.display_value();
            match formatter {
                None => v.with_unit(exif_source).to_string(),
                Some(t) => {
                    let mut vs  = v.to_string();
                    t(vs)
                }
            }
        }
    }
}

