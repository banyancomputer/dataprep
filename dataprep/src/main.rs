#![feature(io_error_more)]
#![feature(buf_read_has_data_left)]
#![deny(unused_crate_dependencies)]

use clap::Parser;
use dataprep_pipelines::pipeline::pack_pipeline::pack_pipeline;
use dataprep_pipelines::pipeline::unpack_pipeline::unpack_pipeline;
use std::path::PathBuf;
use std::time::Instant;

mod cli;

#[tokio::main]
async fn main() {
    // Parse command line arguments. see args.rs
    let cli = cli::Args::parse();
    match cli.command {
        cli::Commands::Pack {
            input_dir,
            output_dir,
            manifest_file,
            target_chunk_size,
            follow_links,
            output_stats,
        } => {
            let now = Instant::now();
            pack_pipeline(
                input_dir,
                output_dir,
                manifest_file,
                target_chunk_size,
                follow_links,
            )
            .await
            .unwrap();
            let elapsed = now.elapsed().as_secs_f64();
            // If output_stats is true, output a stats.txt file
            if output_stats {
                // Create the stats file
                let stats_file = PathBuf::from("stats.txt");
                // Write the stats to the file
                std::fs::write(stats_file, format!("Completed in: {:?} s", elapsed)).unwrap();
            }
        }
        cli::Commands::Unpack {
            input_dir,
            manifest_file,
            output_dir,
            output_stats,
        } => {
            let now = Instant::now();
            unpack_pipeline(input_dir, output_dir, manifest_file)
                .await
                .unwrap();
            let elapsed = now.elapsed().as_secs_f64();
            // If output_stats is true, output a stats.txt file
            if output_stats {
                // Create the stats file
                let stats_file = PathBuf::from("stats.txt");
                // Write the stats to the file
                std::fs::write(stats_file, format!("Completed in: {:?} s", elapsed)).unwrap();
            }
        }
    }
}
