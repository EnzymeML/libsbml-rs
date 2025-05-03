//! Build script for SBML Rust bindings
//!
//! This script handles:
//! 1. Finding the required libraries (libxml2 and libSBML) using vcpkg
//! 2. Generating Rust bindings to the C++ code using autocxx
//! 3. Configuring the build environment and linking
//!
//! The script requires vcpkg to be properly configured to find the dependencies.
//! Use `cargo install cargo-vcpkg && cargo vcpkg build` to install all dependencies.

use std::process::Command;

use autocxx_build::BuilderError;

/// Main build script function that orchestrates the build process
///
/// This function:
/// 1. Finds libSBML using vcpkg
/// 2. Generates Rust bindings using autocxx
/// 3. Configures the build environment and linking
/// 4. Handles platform-specific requirements (like libiconv on macOS)
///
/// # Returns
/// * `Result<(), BuilderError>` - Success or error result
fn main() -> Result<(), BuilderError> {
    // Ensure cargo rebuilds if this build script changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    // Set up vcpkg and get required libraries
    let libsbml = setup_vcpkg()?;

    // Configure autocxx to generate Rust bindings
    let rs_file = "src/lib.rs";

    // Build the C++ wrapper code and bindings
    let mut b = autocxx_build::Builder::new(
        rs_file,
        std::iter::once(".")
            .chain(libsbml.include_paths.iter().map(|p| p.to_str().unwrap()))
            .collect::<Vec<_>>(),
    )
    .build()?;

    // Ensure C++17 is used for compilation and disable warnings
    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-w") // Disable all warnings
        .compile("sbmlrs");

    link_lib(&libsbml.cargo_metadata);

    // Add BCrypt for Windows build (needed by libxml2)
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=bcrypt");
    }

    Ok(())
}

/// Sets up vcpkg and retrieves required dependencies
///
/// This function:
/// 1. Ensures cargo-vcpkg is installed
/// 2. Runs vcpkg build to install dependencies
/// 3. Finds and returns the libsbml package
///
/// # Returns
/// * `Result<vcpkg::Library, BuilderError>` - The libsbml library information
fn setup_vcpkg() -> Result<vcpkg::Library, BuilderError> {
    // Check if cargo-vcpkg is installed by checking if it's in the list of installed crates
    let cargo_vcpkg_installed = Command::new("cargo")
        .args(["--list"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).contains("vcpkg"))
        .unwrap_or(false);

    if !cargo_vcpkg_installed {
        // Install cargo-vcpkg if not found
        println!("cargo:warning=Installing cargo-vcpkg...");
        Command::new("cargo")
            .args(["install", "cargo-vcpkg"])
            .status()
            .expect("Failed to install cargo-vcpkg");
    }

    // Get the target directory first so we can configure vcpkg
    let target_dir = get_vcpkg_dir();

    // Create the vcpkg directory if it doesn't exist
    std::fs::create_dir_all(&target_dir).expect("Failed to create vcpkg directory");

    // Set VCPKG_ROOT environment variable for the cargo vcpkg build command
    std::env::set_var("VCPKG_ROOT", &target_dir);

    // Run cargo vcpkg build to install dependencies
    Command::new("cargo")
        .args(["vcpkg", "build"])
        .status()
        .expect("Failed to run cargo vcpkg build");

    let libsbml = vcpkg::Config::new()
        .vcpkg_root(target_dir)
        .find_package("libsbml")
        .expect("Failed to find libsbml. Use `cargo install cargo-vcpkg && cargo vcpkg build` to install all dependencies.");

    Ok(libsbml)
}

/// Helper function to process and print cargo metadata for linking libraries
///
/// # Arguments
/// * `cargo_metadata` - A slice of strings containing cargo metadata directives
fn link_lib(cargo_metadata: &[String]) {
    for metadata in cargo_metadata {
        println!("{}", metadata);
    }
}

/// Helper function to get the vcpkg directory
///
/// This function:
/// 1. Checks if CARGO_MANIFEST_DIR is set
/// 2. If set, constructs the path to the vcpkg directory relative to the project root
/// 3. If not set, uses a hardcoded path ("target/vcpkg")
///
/// # Returns
/// * `std::path::PathBuf` - The path to the vcpkg directory
fn get_vcpkg_dir() -> std::path::PathBuf {
    // For publishing and regular builds, CARGO_MANIFEST_DIR should be set
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // When publishing, we need to install dependencies in the temporary package dir
    std::path::Path::new(&manifest_dir).join("target/vcpkg")
}
