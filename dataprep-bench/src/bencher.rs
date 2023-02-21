use anyhow::{Error, Result};
use core::ops::FnMut;
use fake_file::utils::fs_utils::ensure_path_exists_and_is_dir;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::time::Instant;
use std::{fs, path::PathBuf};

/// Enum for describing what type of measurement to take
#[derive(Serialize, Deserialize, Clone, Debug, strum::Display)]
pub enum Measurement {
    /// Measure the Throughput of the given function, in Bytes per second
    Throughput(usize, usize),
}

pub struct Bencher {
    /// A setup function to run before each iteration
    // setup: Option<Box<dyn Fn() -> ()>>,
    /// A teardown function to run after each iteration
    // teardown: Option<Box<dyn Fn() -> ()>>,
    /// An id for the benchmark
    pub id: Option<String>,
    /// Configure where to write the benchmark results
    pub output: Option<PathBuf>,
    // TODO: Implement history
    // Whether or not to run the benchmark as a baseline
    // prev: Option<Measurement>,
    pub measurement: Option<Measurement>,
}

/// A Bencher for long running benchmarks!
impl Bencher {
    /// Create a new Bencher
    /// # Arguments
    /// * `func` - The function to benchmark
    /// * `iterations` - The number of times to run the function
    pub fn new() -> Self {
        Self {
            // func,
            // iterations,
            // setup: None,
            // teardown: None,
            id: None,
            output: None,
            // baseline: false,
            measurement: None,
        }
    }

    // /// Set the setup function
    // /// # Arguments
    // /// * `setup` - The setup function to run before each iteration
    // /// # Returns
    // /// A reference to the Bencher
    // pub fn with_setup(&mut self, setup: Box<dyn Fn() -> ()>) -> &mut Self {
    //     self.setup = Some(setup);
    //     self
    // }
    //
    // /// Set the teardown function
    // /// # Arguments
    // /// * `teardown` - The teardown function to run after each iteration
    // /// # Returns
    // /// A reference to the Bencher
    // pub fn with_teardown(&mut self, teardown: Box<dyn Fn() -> ()>) -> &mut Self {
    //     self.teardown = Some(teardown);
    //     self
    // }

    /// Set the id of the benchmark
    /// # Arguments
    /// * `id` - The id of the benchmark
    /// # Returns
    /// A reference to the Bencher
    pub fn with_id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
        self
    }

    /// Set the output of the benchmark
    /// # Arguments
    /// * `output` - The output of the benchmark
    /// # Returns
    /// A reference to the Bencher
    pub fn with_output(&mut self, output: PathBuf) -> &mut Self {
        self.output = Some(output);
        self
    }

    // /// Set if we should run the benchmark as a baseline
    // /// # Arguments
    // /// * `baseline` - Whether or not to run the benchmark as a baseline
    // /// # Returns
    // /// A reference to the Bencher
    // pub fn with_comparison(&mut self, baseline: bool) -> &mut Self {
    //     self.baseline = baseline;
    //     self
    // }

    /// Run the benchmark
    /// # Arguments
    /// * `measurement` - The type of measurement to use
    /// * `func` - The function to benchmark
    /// # Returns
    /// The result of the benchmark
    pub fn run(
        &mut self,
        measurement: Measurement,
        mut func: Box<dyn FnMut() -> ()>,
    ) -> Result<(), Error> {
        match measurement {
            Measurement::Throughput(size, _) => self.run_throughput(size, func),
        };
        Ok(())
    }

    /// Write the benchmark results to a file
    /// # Arguments
    /// * `measurement` - The result of the benchmark
    /// # Returns
    /// Ok if the benchmark was written successfully, Err otherwise
    pub fn write(&mut self) -> Result<(), Error> {
        let path = self.get_path();
        //
        // ensure the parent directory exists

        let p = path.parent().unwrap();
        ensure_path_exists_and_is_dir(&p)?;
        // Get the measurement
        let measurement = self.measurement.clone().unwrap();
        // Create the file
        let f = fs::File::create(&path)?;
        // Write the results to the file
        serde_json::to_writer_pretty(f, &measurement)?;
        Ok(())
    }

    /// Run the benchmark and measure the throughput
    /// # Returns
    /// The result of the benchmark
    fn run_throughput(
        &mut self,
        size: usize,
        mut func: Box<dyn FnMut() -> ()>,
    ) -> Result<(), Error> {
        let mut elapsed: Duration = Duration::from_secs(0);
        // for _ in 0..self.iterations {
        // Run the setup function
        // if let Some(setup) = &self.setup {
        //     (setup)();
        // }
        // Run the benchmark
        let start = Instant::now();
        (func)();
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
            self.id.clone().unwrap(),
            throughput
        );

        self.measurement = Some(Measurement::Throughput(size, elapsed as usize));
        Ok(())
    }

    fn get_path(&self) -> PathBuf {
        let mut path: PathBuf = if let Some(output) = &self.output {
            output.clone()
        } else {
            PathBuf::from("../target/benchmarks/")
        };

        if let Some(id) = &self.id {
            // Get the path to the benchmark file
            path.push(format!("{}.json", id));
        } else {
            path.push("benchmark.json");
        }
        path
    }
}
