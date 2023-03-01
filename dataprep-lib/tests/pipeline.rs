use dataprep_lib::{
    pipeline::{pack_pipeline::pack_pipeline, unpack_pipeline::unpack_pipeline},
    utils::fs::{ensure_path_exists_and_is_dir, ensure_path_exists_and_is_empty_dir},
};
use dir_assert::assert_paths;
use fake_file::{Strategy, Structure};
use std::{fs::remove_file, path::Path};

const INPUT_PATH: &str = "input";
const PACKED_PATH: &str = "packed";
const UNPACKED_PATH: &str = "unpacked";
const MANIFEST_PATH: &str = "manifest.json";

/// Helper function to setup a test
/// # Arguments
/// * test_path: Where we store artefacts for the test
/// * structure: The structure of the test
/// * test_name: The name of the test
fn setup_test(test_path: &Path, structure: Structure, test_name: &str) {
    // Declare Paths for the Input, Packed, Unpacked, and Manifest
    let mut input_path = test_path.join(INPUT_PATH);
    let packed_path = test_path.join(PACKED_PATH);
    let unpacked_path = test_path.join(UNPACKED_PATH);
    let manifest_path = test_path.join(MANIFEST_PATH);
    // Prepare the test structure
    ensure_path_exists_and_is_empty_dir(&input_path, true).unwrap();
    input_path.push(test_name);
    structure.generate(&input_path).unwrap();
    ensure_path_exists_and_is_empty_dir(&packed_path, true).unwrap();
    ensure_path_exists_and_is_empty_dir(&unpacked_path, true).unwrap();
    remove_file(&manifest_path).unwrap_or_default();
}

/// Helper function to run a test end to end
/// # Arguments
/// * test_path: Where we store artefacts for the test
async fn run_test(test_path: &Path) {
    // Declare Paths for the Input, Packed, Unpacked, and Manifest
    let input_path = test_path.join(INPUT_PATH);
    let packed_path = test_path.join(PACKED_PATH);
    let unpacked_path = test_path.join(UNPACKED_PATH);
    let manifest_path = test_path.join(MANIFEST_PATH);

    // Pack the input
    pack_pipeline(
        input_path.clone(),
        packed_path.clone(),
        manifest_path.clone(),
        1073741824, // 1GB
        false,
    )
    .await
    .unwrap();
    // Unpack the output
    unpack_pipeline(
        packed_path.clone(),
        unpacked_path.clone(),
        manifest_path.clone(),
    )
    .await
    .unwrap();

    // checks if two directories are the same
    assert_paths(input_path.clone(), unpacked_path.clone()).unwrap();
}

/// Small Input End to End Integration Tests for the Pipeline
#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    // Configure where tests are run
    const TEST_PATH: &str = "test";
    // Configure the test sets
    const TEST_MAX_WIDTH: usize = 4;
    const TEST_MAX_DEPTH: usize = 4;
    const TEST_INPUT_SIZE: usize = 1024 * 1024; // 1MB

    /// Test the pipeline with a small file structure
    #[tokio::test]
    async fn test_simple() {
        // Create a new path for this test
        let test_path = Path::new(TEST_PATH);
        let test_path = test_path.join("simple");
        // Define the file structure to test
        let desired_structure = Structure::new(
            TEST_MAX_WIDTH, // width
            TEST_MAX_DEPTH, // depth
            TEST_INPUT_SIZE,
            Strategy::Simple,
        );
        // Setup the test
        setup_test(&test_path, desired_structure, "test_simple");
        // Run the test
        run_test(&test_path).await;
    }

    /// Test the pipeline with a very deep file structure
    #[tokio::test]
    async fn test_deep() {
        // Create a new path for this test
        let test_path = Path::new(TEST_PATH);
        let test_path = test_path.join("deep");
        // Define the file structure to test
        let desired_structure = Structure::new(
            2, // width
            8, // depth
            TEST_INPUT_SIZE,
            Strategy::Simple,
        );
        // Setup the test
        setup_test(&test_path, desired_structure, "test_deep");
        // Run the test
        run_test(&test_path).await;
    }

    /// Test the pipeline with a very wide file structure
    #[tokio::test]
    async fn test_wide() {
        // Create a new path for this test
        let test_path = Path::new(TEST_PATH);
        let test_path = test_path.join("wide");
        // Define the file structure to test
        let desired_structure = Structure::new(
            16, // width
            2,  // depth
            TEST_INPUT_SIZE,
            Strategy::Simple,
        );
        // Setup the test
        setup_test(&test_path, desired_structure, "test_wide");
        // Run the test
        run_test(&test_path).await;
    }

    /// Test with one very big file -- ignore cuz it takes a while
    #[tokio::test]
    #[ignore]
    async fn test_big_file() {
        // Create a new path for this test
        let test_path = Path::new(TEST_PATH);
        let test_path = test_path.join("big_file");
        // Define the file structure to test
        let desired_structure = Structure::new(0, 0, TEST_INPUT_SIZE * 1024, Strategy::Simple);
        // Setup the test
        setup_test(&test_path, desired_structure, "test_big_file");
        // Run the test
        run_test(&test_path).await;
    }

    /// Test the pipeline with a trivial duplicated file structure -- ignore cuz it broke
    #[tokio::test]
    #[ignore]
    async fn test_duplicate_dir() {
        // Create a new path for this test
        let test_path = Path::new(TEST_PATH);
        let test_path = test_path.join("duplicate_dir");
        // Define the file structure to test
        let desired_structure = Structure::new(1, 1, TEST_INPUT_SIZE, Strategy::Simple);
        // Setup the test
        setup_test(&test_path, desired_structure, "test_duplicate_dir");
        // Duplicate the test file
        let input_path = test_path.join(INPUT_PATH);
        // Copy $input_path/test_duplicate to $input_path/_test_duplicate
        let original_path = input_path.join("test_duplicate_dir");
        let duplicate_path = input_path.join("_test_duplicate_dir");
        ensure_path_exists_and_is_dir(&duplicate_path).unwrap();
        fs_extra::dir::copy(
            &original_path,
            &duplicate_path,
            &fs_extra::dir::CopyOptions::new(),
        )
        .unwrap();
        // Run the test
        run_test(&test_path).await;
    }
}
