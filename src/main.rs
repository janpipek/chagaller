mod cli;
mod gallery;
mod pages;
mod image;
mod render;

use std::path::PathBuf;

use crate::cli::{Cli, Commands};
use crate::gallery::{Gallery, GalleryOpts};
use clap::Parser;
use crate::render::render_gallery;

fn process(input_dir: &PathBuf, output_dir: &PathBuf) {
    let dir_str = input_dir.display();
    println!("Processing '{dir_str}'.");

    let opts = GalleryOpts {
        max_width: 1600,
        max_height: 1600,
        thumbnail_size: 128,
    };

    let gallery = Gallery::from_input_dir(input_dir);
    render_gallery(&gallery, &output_dir, &opts);
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
