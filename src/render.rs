use std::fs;
use std::fs::File;
use std::io::BufWriter;
use image::imageops::FilterType;
use image::ImageReader;
use crate::pages::IndexTemplate;
use std::path::PathBuf;
use crate::image::Image;
use askama::Template;
use std::io::Write;

pub fn render_gallery(gallery: &crate::gallery::Gallery, output_dir: &PathBuf, opts: &crate::gallery::GalleryOpts) {
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

pub fn render_images(image: &Image, output_dir: &PathBuf, opts: &crate::gallery::GalleryOpts) {
    let image_path = &image.source_path;
    let target_path = get_target_path(output_dir, image_path);
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    let scaled_img = img.resize(opts.max_width, opts.max_height, FilterType::Nearest);
    scaled_img.save(&target_path).ok();

    let thumbnail_path = get_thumbnail_path(output_dir, image_path);
    let thumbnail_img = img.resize(128, 128, FilterType::Nearest);
    thumbnail_img.save(&thumbnail_path).ok();
}

pub fn render_gallery_page(gallery: &crate::gallery::Gallery, output_dir: &PathBuf) {
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
