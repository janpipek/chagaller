
use std::path::{Path, PathBuf};
use crate::image::Image;

pub struct GalleryOpts {
    pub max_width: u32,
    pub max_height: u32,
}

pub struct Gallery {
    pub title: String,
    pub images: Vec<Image>,
}

impl Gallery {
    pub fn from_input_dir(input_dir: &Path) -> Self {
        let images = find_image_files(input_dir);
        Self {
            images: images.iter().map(|p| Image::from_source_path(p)).collect(),
            title: String::from("Gallery"),
        }
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
