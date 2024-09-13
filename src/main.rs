mod cli;
mod gallery;
mod pages;

use std::path::PathBuf;

use crate::cli::{Cli, Commands};
use crate::gallery::{render_gallery, Gallery};
use clap::Parser;
use gallery::GalleryOpts;

fn process(input_dir: &PathBuf, output_dir: &PathBuf) {
    let dir_str = input_dir.display();
    println!("Processing '{dir_str}'.");

    let opts = GalleryOpts {
        max_width: 1600,
        max_height: 1600,
    };

    let gallery = Gallery::from_input_dir(input_dir);
    render_gallery(&gallery, &output_dir, &opts);
}

fn main() {
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
