use std::fs;
use std::path::PathBuf;

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

pub fn create_gallery(output_dir: &PathBuf, source_images: &Vec<PathBuf>) {
    if output_dir.exists() && !output_dir.is_dir() {
        return;
    }

    fs::create_dir_all(output_dir).ok();

    for image_path in source_images {
        render_image(image_path, output_dir);
    }
}

pub fn render_image(image_path: &PathBuf, output_dir: &PathBuf) {
    let name = image_path.file_name().unwrap();
    let target_path = output_dir.join(name);
    fs::copy(image_path, target_path);
}
