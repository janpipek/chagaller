mod cli;
mod gallery;

use std::path::PathBuf;

use crate::cli::{Cli, Commands};
use crate::gallery::{create_gallery, find_image_files};
use clap::Parser;

fn process(input_dir: &PathBuf, output_dir: &PathBuf) {
    let dir_str = input_dir.display();
    println!("Processing '{dir_str}'.");

    let image_files = find_image_files(input_dir);
    for f in &image_files {
        println!("{}", f.display());
    }

    create_gallery(output_dir, &image_files);
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
