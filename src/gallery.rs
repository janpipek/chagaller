use image::imageops::FilterType;
use image::ImageReader;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use askama::Template;
use crate::pages::IndexTemplate;

pub fn find_image_files(input_dir: &PathBuf) -> Vec<PathBuf> {
    let allowed_extensions = vec!["jpg", "jpeg"];
    let mut result = vec![];
    for item in input_dir.read_dir().expect("Cannot read from directory.") {
        let path = item.unwrap().path();
        let ext = path.extension().unwrap_or_default().to_str().unwrap();
        if allowed_extensions.contains(&ext) {
            result.push(path);
        }
    }
    return result;
}

pub fn create_gallery(output_dir: &PathBuf, source_images: &Vec<PathBuf>, opts: &GalleryOpts) {
    if output_dir.exists() && !output_dir.is_dir() {
        return;
    }

    fs::create_dir_all(output_dir).ok();
    fs::create_dir(output_dir.join("images")).ok();
    fs::create_dir(output_dir.join("thumbnails")).ok();

    for image_path in source_images {
        process_image(image_path, output_dir, opts);
    }

    create_gallery_page(output_dir, source_images);
}

pub fn process_image(image_path: &PathBuf, output_dir: &PathBuf, opts: &GalleryOpts) {
    let target_path = get_target_path(output_dir, image_path);
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    let scaled_img = img.resize(opts.max_width, opts.max_height, FilterType::Nearest);
    scaled_img.save(&target_path).ok();

    let thumbnail_path = get_thumbnail_path(output_dir, image_path);
    let thumbnail_img = img.resize(128, 128, FilterType::Nearest);
    thumbnail_img.save(&thumbnail_path).ok();
}

pub fn create_gallery_page(output_dir: &PathBuf, source_images: &Vec<PathBuf>) {
    let page_path = output_dir.join("index.html");

    let f = File::create(page_path).expect("Unable to create file");
    let mut output = BufWriter::new(f);

    let index = IndexTemplate { title: "gallery"};
    write!(output, "{}", index.render().unwrap());
}

pub fn get_thumbnail_path(output_dir: &PathBuf, image_path: &PathBuf) -> PathBuf {
    let name = image_path.file_name().unwrap();
    let mut thumbnail_path = output_dir.join("thumbnails").join(name);
    thumbnail_path.set_extension("jpg");
    return thumbnail_path;
}

pub fn get_target_path(output_dir: &PathBuf, image_path: &PathBuf) -> PathBuf {
    let name = image_path.file_name().unwrap();
    let target_path = output_dir.join("images").join(name);
    return target_path;
}
