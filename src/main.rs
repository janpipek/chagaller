mod cli;
mod gallery;
mod image;
mod pages;
mod render;

use std::path::Path;

use crate::cli::{Cli, Commands};
use crate::gallery::{Gallery, GalleryOpts};
use crate::render::render_gallery;
use clap::Parser;

fn process(input_dir: &Path, output_dir: &Path) {
    let dir_str = input_dir.display();
    println!("Processing '{dir_str}'.");

    let opts = GalleryOpts {
        max_width: 1600,
        max_height: 1600,
        thumbnail_size: 128,
    };

    let gallery = Gallery::from_input_dir(input_dir);
    let image_count = gallery.image_count();
    if image_count > 0 {
        log::info!("Found {} images.", gallery.image_count());
        render_gallery(&gallery, output_dir, &opts);
        log::info!("Done.");
    } else {
        log::info!("No images found.");
    }
}

fn main() {
    env_logger::init();
    let args = Cli::parse();

    match &args.command {
        Commands::Create {
            input_dir,
            output_dir,
        } => {
            process(input_dir, output_dir);
        }
    }
}
