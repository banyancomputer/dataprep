use clap::{Parser, Subcommand};
use std::path::PathBuf;

// Note (amiller68): We keep this so we can use command line arguments for dataprep
// We will use env::args for the benchmarking tool

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    #[clap(about = "Pack a directory")]
    Pack {
        #[arg(short, long, help = "input directories and files")]
        input_dir: PathBuf,
        #[arg(short, long, help = "output directory")]
        output_dir: PathBuf,
        #[arg(short, long, help = "manifest file location")]
        manifest_file: PathBuf,
        #[arg(short, long, help = "target chunk size", default_value = "1073741824")]
        target_chunk_size: u64,
        #[arg(short, long, help = "follow symlinks", default_value = "false")]
        follow_links: bool,
    },
    #[clap(about = "Unpack a directory")]
    Unpack {
        /// input file root
        #[arg(short, long, help = "input directories and files")]
        input_dir: PathBuf,

        /// where to get the manifest file
        #[arg(short, long, help = "manifest file location")]
        manifest_file: PathBuf,

        /// output directory to repopulate with reinflated files
        #[arg(short, long, help = "output directory")]
        output_dir: PathBuf,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
