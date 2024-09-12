use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)] // requires `derive` feature
#[command(name = "chagaller")]
#[command(about = "Gallery creator in Rust.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Create {
        /// Directory with the image files
        input_dir: PathBuf,

        /// Directory to write the gallery into
        output_dir: PathBuf,
    },
}
