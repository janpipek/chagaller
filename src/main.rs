mod cli;

use std::path::PathBuf;

use crate::cli::Cli;
use crate::cli::Commands;
use clap::Parser;

fn process(input_dir: &PathBuf) {
    let dir_str = input_dir.display();
    println!("Processing '{dir_str}'.");
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Create { input_dir } => {
            process(input_dir);
        }
    }
}
