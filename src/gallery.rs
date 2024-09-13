use image::imageops::FilterType;
use image::ImageReader;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use askama::Template;
use crate::pages::IndexTemplate;

pub struct GalleryOpts {
    pub max_width: u32,
    pub max_height: u32,
}

pub struct Image {
    pub source_path: PathBuf,
    pub title: String,
}

pub struct Gallery {
    pub title: String,
    pub images: Vec<Image>,
}

impl Image {
    pub fn from_source_path(source_path: &Path) -> Self {
        Self {
            source_path: PathBuf::from(source_path),
            title: String::from("Image"),
        }
    }
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

pub fn render_gallery(gallery: &Gallery, output_dir: &PathBuf, opts: &GalleryOpts) {
    if output_dir.exists() && !output_dir.is_dir() {
        return;
    }

    fs::create_dir_all(output_dir).ok();
    fs::create_dir(output_dir.join("images")).ok();
    fs::create_dir(output_dir.join("thumbnails")).ok();

    for image in gallery.images.iter() {
        render_images(image, output_dir, opts);
    }

    render_gallery_page(&gallery, output_dir);
}

pub fn render_images(image: &Image, output_dir: &PathBuf, opts: &GalleryOpts) {
    let image_path = &image.source_path;
    let target_path = get_target_path(output_dir, image_path);
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    let scaled_img = img.resize(opts.max_width, opts.max_height, FilterType::Nearest);
    scaled_img.save(&target_path).ok();

    let thumbnail_path = get_thumbnail_path(output_dir, image_path);
    let thumbnail_img = img.resize(128, 128, FilterType::Nearest);
    thumbnail_img.save(&thumbnail_path).ok();
}

pub fn render_gallery_page(gallery: &Gallery, output_dir: &PathBuf) {
    let page_path = output_dir.join("index.html");

    let f = File::create(page_path).expect("Unable to create file");
    let mut output = BufWriter::new(f);

    let index = IndexTemplate { title: "gallery"};
    write!(output, "{}", index.render().unwrap()).ok();
}

pub fn get_thumbnail_path(output_dir: &PathBuf, image_path: &PathBuf) -> PathBuf {
    let name = image_path.file_name().unwrap();
    let mut thumbnail_path = output_dir.join("thumbnails").join(name);
    thumbnail_path.set_extension("jpg");
    thumbnail_path
}

pub fn get_target_path(output_dir: &PathBuf, image_path: &PathBuf) -> PathBuf {
    let name = image_path.file_name().unwrap();
    let target_path = output_dir.join("images").join(name);
    target_path
}
