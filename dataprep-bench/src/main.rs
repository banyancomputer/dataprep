#![feature(io_error_more)]
#![feature(buf_read_has_data_left)]
// #![deny(unused_crate_dependencies)]
#![feature(async_closure)]

use clap::Parser;
use dataprep_pipelines::pipeline::pack_pipeline::pack_pipeline;
use dataprep_pipelines::pipeline::unpack_pipeline::unpack_pipeline;
use fs_extra;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod bencher;
mod cli;

#[tokio::main]
// TODO: (amiller68): This is horrible
async fn main() {
    // Parse dataprep command line arguments. see cli.rs
    let dataprep_args = cli::Args::parse();
    // Match the command enum
    match dataprep_args.command {
        cli::Commands::Pack {
            input_dir,
            output_dir,
            manifest_file,
            target_chunk_size,
            follow_links,
        } => {
            println!("Packing");
            let input_dir_name = input_dir.file_name().unwrap().to_str().unwrap().to_string();
            let size = fs_extra::dir::get_size(&input_dir).unwrap() as usize;
            // Create a new benchmark
            let m = bencher::Measurement::Throughput(size, 0); // In the form of (input_size, time)
            let mut t = bencher::Bencher::new();
            let b = t.with_id(input_dir_name.to_string());
            // TODO: Figure out how to run the pack_pipeline function inside the context of the bencher
            // b.run(
            //     m,
            //     Box::new(async || {
            //         // sleep(Duration::from_secs(5));
            //         pack_pipeline(
            //             input_dir,
            //             output_dir,
            //             manifest_file,
            //             target_chunk_size,
            //             follow_links,
            //         )
            //         .await
            //         .unwrap();
            //     }),
            // )
            // .unwrap();
            let mut elapsed: Duration = Duration::from_secs(0);
            let start = Instant::now();
            // (func)();
            pack_pipeline(
                input_dir,
                output_dir,
                manifest_file,
                target_chunk_size,
                follow_links,
            )
            .await
            .unwrap();
            let end = Instant::now();

            // Run the teardown function
            // if let Some(teardown) = &self.teardown {
            //     (teardown)();
            // }
            elapsed += end - start;
            // }
            // Rebind to the average elapsed time
            // let elapsed = elapsed.as_secs_f64() / self.iterations as f64;
            // Calculate the throughput
            let elapsed = elapsed.as_secs_f64();
            let throughput = (size as f64 / elapsed) as usize;
            // Log the number of second
            // Log the results
            println!(
                "Benchmark: {} - Throughput: {} bytes/s",
                b.id.clone().unwrap(),
                throughput
            );
            b.measurement = Some(bencher::Measurement::Throughput(size, elapsed as usize));
            b.write();
        }
        cli::Commands::Unpack {
            input_dir,
            manifest_file,
            output_dir,
        } => {
            println!("Unpacking");
            let input_dir_name = input_dir.file_name().unwrap().to_str().unwrap();
            let size = fs_extra::dir::get_size(&input_dir).unwrap() as usize;
            // Create a new benchmark
            let m = bencher::Measurement::Throughput(size, 0); // In the form of (input_size, time)
            let mut t = bencher::Bencher::new();
            let b = t.with_id(input_dir_name.to_string());
            // b.run(
            //     m,
            //     Box::new(|| {
            //         sleep(Duration::from_secs(5));
            //     }),
            // )
            // .unwrap();
            // b.write();
            let mut elapsed: Duration = Duration::from_secs(0);
            let start = Instant::now();
            // (func)();
            unpack_pipeline(input_dir, output_dir, manifest_file)
                .await
                .unwrap();
            let end = Instant::now();

            // Run the teardown function
            // if let Some(teardown) = &self.teardown {
            //     (teardown)();
            // }
            elapsed += end - start;
            // }
            // Rebind to the average elapsed time
            // let elapsed = elapsed.as_secs_f64() / self.iterations as f64;
            // Calculate the throughput
            let elapsed = elapsed.as_secs_f64();
            let throughput = (size as f64 / elapsed) as usize;
            // Log the number of second
            // Log the results
            println!(
                "Benchmark: {} - Throughput: {} bytes/s",
                b.id.clone().unwrap(),
                throughput
            );
            b.measurement = Some(bencher::Measurement::Throughput(size, elapsed as usize));
            b.write();
        }
    };
}
