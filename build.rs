//! Build script for SBML Rust bindings
//!
//! This script handles:
//! 1. Building the required C++ libraries (libxml2 and libSBML) from source
//! 2. Generating Rust bindings to the C++ code using autocxx
//! 3. Configuring the build environment and linking
//!
//! The script requires CMake to be installed on the system for building the C++ libraries.

/// Name of the SBML library
const LIBSBML_NAME: &str = "sbml";

/// Path to the libSBML source code
const LIBSBML_PATH: &str = "vendors/libsbml";

/// Path to the libSBML dependencies source code
const LIBSBML_DEPENDENCY_DIR: &str = "vendors/libsbml-dependencies";

/// Name of the Expat library file on Windows
const EXPAT_WINDOWS_LIB: &str = "libexpat.lib";

/// Name of the zlib library file on Windows
const ZLIB_WINDOWS_LIB: &str = "zdll.lib";

/// Whether to build with libxml2 support (disabled in favor of Expat)
const WITH_LIBXML: &str = "OFF";

/// Whether to build with Expat XML parser support
const WITH_EXPAT: &str = "ON";

/// Main build script function that orchestrates the build process
///
/// This function:
/// 1. Builds libSBML dependencies if on Windows
/// 2. Builds the libSBML library
/// 3. Generates Rust bindings using autocxx
/// 4. Configures the build environment and linking
///
/// # Returns
/// * `miette::Result<()>` - Success or error result
fn main() -> miette::Result<()> {
    // Ensure cargo rebuilds if this build script changes
    println!("cargo:rerun-if-changed=build.rs");

    // Build and link libSBML dependencies
    let dep_build = if cfg!(target_os = "windows") {
        build_and_link_sbml_deps()?
    } else {
        String::new()
    };

    // Build and link libSBML
    let sbml_build = build_and_link_libsbml(&dep_build)?;

    // Print the contents of the sbml_build directory
    print_dir_contents(&sbml_build)?;

    // Configure autocxx to generate Rust bindings
    let rs_file = "src/lib.rs";

    // Point to the libSBML headers
    let sbml_include = format!("{}/include", sbml_build);
    let lib_root = ".";

    // Build the C++ wrapper code and bindings
    let mut b = autocxx_build::Builder::new(rs_file, [lib_root, &sbml_include]).build()?;

    // Ensure C++20 is used for compilation
    b.flag_if_supported("-std=c++17").compile("sbmlrs");

    Ok(())
}

/// Helper function to build and link the libSBML library using CMake
///
/// This function handles the platform-specific build configuration:
/// - On Windows, it configures paths to Expat and zlib dependencies
/// - On MacOS/Linux, it uses system libraries
///
/// # Arguments
/// * `dep_build` - Path to the dependency build directory (used on Windows)
///
/// # Returns
/// * `miette::Result<String>` - Build directory path on success, error on failure
fn build_and_link_libsbml(dep_build: &str) -> miette::Result<String> {
    let dst = if cfg!(target_os = "windows") {
        println!("cargo:warning=Building libSBML for Windows");
        // In order to build for windows, we need to carefully tell CMake
        // where to find the libraries and headers for libexpat and zlib.
        // This is necessary because the libraries are not installed in the
        // system directories by default. Unlinke MacOS and Linux kernels
        cmake::Config::new(LIBSBML_PATH)
            .static_crt(true)
            .profile("Release")
            .define("WITH_STATIC_RUNTIME", "ON")
            .define("WITH_LIBXML", WITH_LIBXML)
            .define("WITH_EXPAT", WITH_EXPAT)
            //
            // Define the paths to the libraries and headers for libexpat and zlib
            //
            .define("EXPAT_INCLUDE_DIR", format!("{}/include", dep_build))
            .define(
                "EXPAT_LIBRARY",
                format!("{}/lib/{}", dep_build, EXPAT_WINDOWS_LIB),
            )
            //
            // Define the path to the library and headers for zlib
            //
            .define("ZLIB_INCLUDE_DIR", format!("{}/include", dep_build))
            .define(
                "ZLIB_LIBRARY",
                format!("{}/lib/{}", dep_build, ZLIB_WINDOWS_LIB),
            )
            //
            // Build static libraries, because dynamic librarier somehow dont work
            //
            .define("BUILD_SHARED_LIBS", "OFF")
            .build()
    } else {
        println!("cargo:warning=Building libSBML for MacOS/Linux");
        // When building for MacOS and Linux, we can just use the system libraries
        cmake::Config::new(LIBSBML_PATH)
            .profile("Release")
            .define("WITH_STATIC_RUNTIME", "OFF")
            .define("WITH_LIBXML", WITH_LIBXML)
            .define("WITH_EXPAT", WITH_EXPAT)
            .build()
    };

    // Configure cargo to link against the built library
    println!("cargo:rustc-link-search={}/lib", dst.display());
    if cfg!(target_os = "windows") {
        // On Windows, we need to link against the static libraries
        // Note: This is where things get tricky, because the libsbml
        // static library is named "libsbml-static" and not "libsbml".
        // which seems to confuse the rustc linker.
        println!("cargo:rustc-link-lib=libsbml-static");
    } else {
        // On MacOS and Linux, we can just link against the dynamic library
        println!("cargo:rustc-link-lib=dylib={}", LIBSBML_NAME);
    }

    Ok(dst.display().to_string())
}

/// Builds and links the libSBML dependencies (Expat and zlib) on Windows
///
/// This function is only used on Windows where we need to build these
/// dependencies from source. On other platforms, system libraries are used.
///
/// # Returns
/// * `miette::Result<String>` - Build directory path on success, error on failure
fn build_and_link_sbml_deps() -> miette::Result<String> {
    println!("cargo:warning=Building libSBML dependencies");

    // Build the dependencies for libSBML
    // We hard-code to EXPAT and ZLIB for now, but in the future this should
    // be made more flexible.
    let dst = cmake::Config::new(LIBSBML_DEPENDENCY_DIR)
        .static_crt(true)
        .profile("Release")
        .define("WITH_STATIC_RUNTIME", "ON")
        .define("EXPAT_MSVC_STATIC_CRT", "ON")
        .define("WITH_EXPAT", "ON")
        .define("WITH_LIBXML", "OFF")
        .define("WITH_ZLIB", "ON")
        .define("WITH_BZIP2", "OFF")
        .define("WITH_CHECK", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    // Configure cargo to link against the built libraries
    // Note: We link against the static libraries, because the dynamic libraries
    // are not working for some reason.
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=libexpat");
    println!("cargo:rustc-link-lib=zdll");

    Ok(dst.display().to_string())
}

/// Recursively prints the contents of a directory for debugging purposes
///
/// # Arguments
/// * `path` - Path to the directory to print
///
/// # Returns
/// * `miette::Result<()>` - Success or error result
fn print_dir_contents(path: &str) -> miette::Result<()> {
    let entries = std::fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ["a", "dylib", "lib"].contains(&ext_str.as_str()) {
                    println!("cargo:info={}", path.display());
                }
            }
        }

        if path.is_dir() {
            print_dir_contents(path.to_str().unwrap())?;
        }
    }
    Ok(())
}
