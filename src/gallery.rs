use crate::image::Image;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug)]
pub struct GalleryOpts {
    pub max_width: u32,
    pub max_height: u32,
    pub thumbnail_size: u32,
}

#[derive(Debug)]
pub struct Gallery  {
    pub title: String,
    pub images: Vec<Image>,
}

#[derive(Deserialize)]
struct GalleryMetaData {
    pub title: String,
}

impl Gallery {
    pub fn from_input_dir(input_dir: &Path) -> Self {
        let meta_data = Self::parse_gallery_yaml(input_dir);
        let images = find_image_files(input_dir);

        Self {
            images: images.iter().map(|p| Image::from_source_path(p)).collect(),
            title: match meta_data {
                None => String::from("Gallery"),
                Some(m) => m.title
            },
        }
    }

    fn parse_gallery_yaml(input_dir: &Path) -> Option<GalleryMetaData> {
        let yaml_path = input_dir.join("gallery.yaml");
        if yaml_path.exists() {
            let file = std::fs::File::open(yaml_path).unwrap();
            let reader = std::io::BufReader::new(file);
            serde_yml::from_reader(reader).unwrap()
        } else {
            None
        }
    }

    pub fn image_count(&self) -> usize {
        self.images.len()
    }
}

fn find_image_files(input_dir: &Path) -> Vec<PathBuf> {
    let allowed_extensions = vec!["jpg", "jpeg"];
    let mut result = vec![];
    for item in input_dir.read_dir().expect("Cannot read from directory.") {
        let path = item.unwrap().path();
        let ext = path.extension().unwrap_or_default().to_str().unwrap();
        if allowed_extensions.contains(&ext) {
            result.push(path);
        }
    }
    result
}
