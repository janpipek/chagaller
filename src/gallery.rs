use image::imageops::FilterType;
use image::ImageReader;
use std::fs;
use std::path::PathBuf;

pub struct GalleryOpts {
    pub max_width: u32,
    pub max_height: u32,
}

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

    // create_gallery_page()
}

pub fn process_image(image_path: &PathBuf, output_dir: &PathBuf, opts: &GalleryOpts) {
    let name = image_path.file_name().unwrap();
    let mut target_path = output_dir.join("images").join(name);
    target_path.set_extension("jpg");

    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    let scaled_img = img.resize(opts.max_width, opts.max_height, FilterType::Nearest);
    scaled_img.save(&target_path).ok();

    // fs::copy(image_path, target_path);
}
