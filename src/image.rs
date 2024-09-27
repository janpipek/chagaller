use chrono::{DateTime, Local};
use exif;
use exif::{Exif, Tag};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug)]
pub struct ExifInfo {
    pub camera: String,
    pub iso: String,
    pub aperture: String,
    pub exposure: String,
    pub focal_length: String,
    pub date_time: DateTime<Local>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetaInfo {
    pub author: Option<String>,
    pub title: Option<String>,
    pub place: Option<String>,
}

impl Default for MetaInfo {
    fn default() -> Self {
        MetaInfo {
            author: None,
            title: None,
            place: None,
        }
    }
}

#[derive(Debug)]
pub struct Image  {
    pub source_path: PathBuf,
    pub exif_info: Option<ExifInfo>,
    pub meta_info: Option<MetaInfo>,
}

impl Image {
    pub fn from_source_path(source_path: &Path) -> Self {
        Self {
            source_path: PathBuf::from(source_path),
            exif_info: extract_exif_info(source_path),
            meta_info: parse_image_yaml(source_path)
        }
    }

    pub fn base_name(&self) -> &str {
        self.source_path.file_stem().unwrap().to_str().unwrap()
    }

    pub fn get_date_time(&self) -> DateTime<Local> {
        match &self.exif_info {
            Some(exif_info) => exif_info.date_time,
            // Use some other info?
            None => Local::now(),
        }
    }

    pub fn get_title(&self) -> Option<String> {
        if let Some(meta_info) = &self.meta_info {
            if let Some(title) = &meta_info.title {
                return Some(title.clone());
            }
        }
        None
    }
}

pub fn extract_exif_info(source_path: &Path) -> Option<ExifInfo> {
    let file = std::fs::File::open(source_path).unwrap();
    let mut reader = BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    match exif_reader.read_from_container(&mut reader) {
        Ok(exif_source) => Some(ExifInfo {
            camera: extract_exif_tag(&exif_source, Tag::Model, None),
            iso: extract_exif_tag(&exif_source, Tag::ISOSpeed, None),
            aperture: extract_exif_tag(
                &exif_source,
                Tag::ApertureValue,
                Some(|x| format!("f/{}", x)),
            ),
            exposure: extract_exif_tag(&exif_source, Tag::ShutterSpeedValue, None),
            focal_length: extract_exif_tag(&exif_source, Tag::FocalLength, None),
            date_time: Local::now(),
        }),
        Err(_) => None,
    }
}

fn extract_exif_tag(exif_source: &Exif, tag: Tag, formatter: Option<fn(String) -> String>) -> String {
    let field = exif_source.get_field(tag, exif::In::PRIMARY);
    match field {
        None => String::from("Unknown"),
        Some(f) => {
            let v = f.display_value();
            match formatter {
                None => v.with_unit(exif_source).to_string(),
                Some(t) => {
                    let vs = v.to_string();
                    t(vs)
                }
            }
        }
    }
}

fn parse_image_yaml(source_path: &Path) -> Option<MetaInfo> {
    let mut yaml_path = source_path.to_path_buf();
    yaml_path.set_extension("yaml");
    if yaml_path.exists() {
        let file = std::fs::File::open(yaml_path).unwrap();
        let reader = std::io::BufReader::new(file);
        serde_yml::from_reader(reader).unwrap()
    } else { None }
}
