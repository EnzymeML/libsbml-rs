//! Build script for SBML, libCombine and related Rust bindings
//!
//! This script handles the complete build process for Rust bindings to SBML and libCombine:
//!
//! 1. Dependency Management:
//!    - Installs cargo-vcpkg if not already installed
//!    - Configures vcpkg to install C++ dependencies (libxml2, libSBML, etc.)
//!    - Alternatively uses pkg-config if system libraries are available
//!
//! 2. Library Building:
//!    - Compiles the Zipper library (needed for OMEX archives)
//!    - Builds libCombine with proper configuration for the current platform
//!    - Ensures all dependencies are properly linked
//!
//! 3. Rust Binding Generation:
//!    - Uses autocxx to generate Rust bindings to the C++ libraries
//!    - Configures include paths and compiler flags
//!    - Handles platform-specific requirements (Windows, macOS, Linux)
//!
//! 4. Optimization:
//!    - Implements smart rebuilding to avoid unnecessary compilation
//!    - Checks if libraries already exist before rebuilding
//!
//! The script supports multiple platforms and handles the complexities of
//! cross-platform C++ library building and linking.

use std::{path::PathBuf, process::Command};

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

    // Add rerun conditions for C++ dependencies to avoid unnecessary rebuilds
    println!("cargo:rerun-if-changed=submodules/zipper");
    println!("cargo:rerun-if-changed=cmake/libcombine_wrapper");

    let (mut include_paths, cargo_metadata, lib_paths) =
        if let Ok((paths, link_paths, lib_paths)) = from_pkg_config("libsbml") {
            // If libsbml is already installed, we don't need to do anything
            println!("cargo:warning=libsbml is already installed");
            (paths, link_paths, lib_paths)
        } else {
            // If libsbml is not installed, we need to install it
            let libsbml = setup_vcpkg()?;
            (
                libsbml.include_paths,
                libsbml.cargo_metadata.clone(),
                libsbml
                    .link_paths
                    .clone()
                    .iter()
                    .map(|p| p.to_str().unwrap().to_string())
                    .collect(),
            )
        };

    let (zlib_include, zlib_library) = if cfg!(target_os = "windows") {
        let target_dir = get_vcpkg_dir();
        let zlib = vcpkg::Config::new()
            .vcpkg_root(target_dir)
            .find_package("zlib")
            .expect("Failed to find zlib. Use `cargo install cargo-vcpkg && cargo vcpkg build` to install all dependencies.");
        link_lib(&zlib.cargo_metadata);
        let include_path = zlib
            .include_paths
            .first()
            .expect("Failed to find zlib include path");
        let lib = zlib
            .found_libs
            .first()
            .expect("Failed to find zlib library");

        (
            Some(include_path.to_str().unwrap().to_string()),
            Some(lib.to_str().unwrap().to_string()),
        )
    } else {
        (None, None)
    };

    // Configure autocxx to generate Rust bindings
    let rs_file = "src/lib.rs";

    // Only build C++ libraries if they haven't been built yet or if sources changed
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let zipper_lib_path = format!("{}/lib/libZipper-static.a", out_dir);
    let combine_lib_path = format!("{}/lib/libCombine-static.a", out_dir);

    if !std::path::Path::new(&zipper_lib_path).exists() {
        println!("cargo:warning=Building zipper library (first time or after clean)");
        build_zipper();
    } else {
        println!("cargo:warning=Zipper library already exists, skipping build");
        println!("cargo:rustc-link-search=native={}/lib", out_dir);
    }

    let libcombine_include_path = if !std::path::Path::new(&combine_lib_path).exists() {
        println!("cargo:warning=Building libCombine (first time or after clean)");
        build_libcombine(&include_paths, &lib_paths, &zlib_include, &zlib_library)
    } else {
        println!("cargo:warning=libCombine already exists, skipping build");
        println!("cargo:rustc-link-search=native={}/lib", out_dir);
        std::path::PathBuf::from(&out_dir).join("include")
    };

    include_paths.push(libcombine_include_path);

    // Build the C++ wrapper code and bindings
    let mut b = autocxx_build::Builder::new(
        rs_file,
        std::iter::once(".")
            .chain(include_paths.iter().map(|p| p.to_str().unwrap()))
            .collect::<Vec<_>>(),
    )
    .build()?;

    // Ensure correct Clang args are used
    println!("cargo:rustc-env=BINDGEN_EXTRA_CLANG_ARGS=-D_GNU_SOURCE");

    // Ensure C++17 is used for compilation and disable warnings
    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-std=gnu++17")
        .flag_if_supported("-w") // Disable all warnings
        .compile("sbmlrs");

    link_lib(&cargo_metadata);

    // Link libCombine dependencies (libSBML) - critical for Linux
    println!("cargo:rustc-link-lib=static=Zipper-static");
    println!("cargo:rustc-link-lib=static=Combine-static");

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
    if !target_dir.exists() {
        std::fs::create_dir_all(&target_dir).expect("Failed to create vcpkg directory");
    }

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

/// Helper function to process and print pkg-config metadata for linking libraries
///
/// # Arguments
/// * `pkg_config` - A string containing the pkg-config name of the library
///
/// # Returns
/// * `Result<(), String>` - Success or error result
fn from_pkg_config(pkg_config: &str) -> Result<(Vec<PathBuf>, Vec<String>, Vec<String>), String> {
    if let Ok(use_vcpkg) = std::env::var("USE_VCPKG") {
        if use_vcpkg.to_lowercase() != "false" {
            return Err("USE_VCPKG is set, so we don't need to use pkg-config".to_string());
        }
    }

    let lib = pkg_config::probe_library(pkg_config).map_err(|e| e.to_string())?;

    for path in lib.include_paths.iter() {
        println!("cargo:include={}", path.to_str().unwrap());
    }

    let mut cargo_metadata = Vec::new();
    for lib in lib.libs {
        cargo_metadata.push(format!("cargo:rustc-link-lib={}", lib));
    }

    let mut link_paths = Vec::new();
    for path in lib.link_paths.iter() {
        link_paths.push(path.to_str().unwrap().to_string());
    }

    Ok((lib.include_paths.clone(), cargo_metadata, link_paths))
}

fn build_zipper() {
    let dst = cmake::Config::new("./submodules/zipper")
        .define("BUILD_TEST", "OFF") // Disable tests
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
}

fn build_libcombine(
    include_paths: &[PathBuf],
    lib_paths: &[String],
    zlib_include: &Option<String>,
    zlib_library: &Option<String>,
) -> PathBuf {
    let mut config = cmake::Config::new("cmake/libcombine_wrapper");

    // Configure dependencies for libCombine
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let libsbml_lib = find_libsbml_lib_file(&lib_paths[0]).expect("Failed to find libsbml library");

    // Point to the libsbml library we just built
    config.define("LIBSBML_LIBRARY", libsbml_lib.to_str().unwrap());
    config.define("LIBSBML_INCLUDE_DIR", include_paths[0].clone());

    // Point to the zlib library we just built
    if let Some(zlib_include) = zlib_include {
        config.define("ZLIB_INCLUDE_DIR", zlib_include);
    }
    if let Some(zlib_library) = zlib_library {
        config.define("ZLIB_LIBRARY", zlib_library);
    }

    // Point to the zipper library we just built
    config.define(
        "ZIPPER_LIBRARY",
        format!("{}/lib/libZipper-static.a", out_dir),
    );
    config.define("ZIPPER_INCLUDE_DIR", format!("{}/include", out_dir));

    // Set XML libraries that libSBML uses
    if cfg!(target_os = "macos") {
        config.define("EXTRA_LIBS", "expat;z;iconv");
    } else if cfg!(target_os = "linux") {
        config.define("EXTRA_LIBS", "expat;z");
    } else if cfg!(target_os = "windows") {
        config.define("EXTRA_LIBS", "expat;zlib");
    }

    // Disable unnecessary features - ESPECIALLY TESTS!
    config.define("WITH_EXAMPLES", "OFF");
    config.define("WITH_CHECK", "OFF");
    config.define("WITH_DOXYGEN", "OFF");
    config.define("LIBCOMBINE_SKIP_SHARED_LIBRARY", "ON");

    // Disable all language bindings
    config.define("WITH_CSHARP", "OFF");
    config.define("WITH_JAVA", "OFF");
    config.define("WITH_PYTHON", "OFF");
    config.define("WITH_PERL", "OFF");
    config.define("WITH_RUBY", "OFF");
    config.define("WITH_R", "OFF");
    config.define("WITH_OCTAVE", "OFF");
    config.define("WITH_MATLAB", "OFF");

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    dst.join("include")
}

fn find_libsbml_lib_file(lib_path: &str) -> Result<PathBuf, String> {
    // Get all files in the lib directory that contain "sbml"
    let entries = std::fs::read_dir(&lib_path)
        .unwrap_or_else(|_| panic!("Failed to read directory: {}", lib_path))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_lowercase().contains("sbml"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    // Return the first match or fall back to the default
    entries
        .first()
        .cloned()
        .ok_or_else(|| format!("No libsbml library found in {}", lib_path))
}
