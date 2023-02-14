use dataprep_pipelines::utils::{
    fs::{FileStructure, FileStructureStrategy},
    test::{pipeline_test, setup_test_structure},
};
use lazy_static::lazy_static;

lazy_static! {
    // Where to work on test outputs when running tests
<<<<<<< HEAD
    static ref TEST_PATH: &'static str = "test_scratch_space";
    static ref INPUT_PATH: &'static str = "test_scratch_space/input";
    static ref OUTPUT_PATH: &'static str = "test_scratch_space/output";
    static ref UNPACKED_PATH:  &'static str = "test_scratch_space/unpacked";
    static ref MANIFEST_FILE_PATH: &'static str = "test_scratch_space/manifest.json";
}

=======
    static ref TEST_PATH: &'static str = "test";
    static ref INPUT_PATH: &'static str = "test/input";
    static ref OUTPUT_PATH: &'static str = "test/packed";
    static ref UNPACKED_PATH:  &'static str = "test/unpacked";
    static ref MANIFEST_FILE_PATH: &'static str = "test/manifest.json";
}

/// Small Input End to End Integration Tests for the Pipeline
>>>>>>> origin/bench
#[cfg(test)]
mod test {
    use super::*;

<<<<<<< HEAD
    #[tokio::test]
    /// A simple end to end integration test of a small file structure
    async fn test_pipeline() {
        // Define the file structure to test
        let desired_structure = FileStructure::new(
            2,                               // width
            2,                               // depth
            1024,                            // target size in bytes (1KB)
            FileStructureStrategy::Balanced, // Balanced
            true,                            // utf8 only
        );

=======
    const TEST_INPUT_SIZE: usize = 1024 * 1024; // 1MB
    const TEST_MAX_WIDTH: usize = 4;
    const TEST_MAX_DEPTH: usize = 4;

    /// Test the pipeline with a small file structure
    #[tokio::test]
    async fn test_pipeline() {
        // Define the file structure to test
        let desired_structure = FileStructure::new(
            TEST_MAX_WIDTH, // width
            TEST_MAX_DEPTH, // depth
            TEST_INPUT_SIZE,
            FileStructureStrategy::Balanced, // Balanced
        );
        println!("Setting up test structure: {:?}", desired_structure);
>>>>>>> origin/bench
        // Setup the test structure
        setup_test_structure(
            &TEST_PATH,
            &INPUT_PATH,
            &OUTPUT_PATH,
            &UNPACKED_PATH,
            &MANIFEST_FILE_PATH,
            desired_structure,
        );
<<<<<<< HEAD
=======
        println!("Running pipeline test");
>>>>>>> origin/bench

        // Run the transform and check
        pipeline_test(
            &INPUT_PATH,
            &OUTPUT_PATH,
<<<<<<< HEAD
            &MANIFEST_FILE_PATH,
            &UNPACKED_PATH,
=======
            &UNPACKED_PATH,
            &MANIFEST_FILE_PATH,
>>>>>>> origin/bench
        )
        .await;
    }
    // TODO: (thea-exe) Add more tests - there might be a problem getting them to run in parallel
}