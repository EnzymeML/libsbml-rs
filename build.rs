//! Build script for SBML Rust bindings
//!
//! This script handles:
//! 1. Building the required C++ libraries (libxml2 and libSBML) from source
//! 2. Generating Rust bindings to the C++ code using autocxx
//! 3. Configuring the build environment and linking
//!
//! The script requires CMake to be installed on the system for building the C++ libraries.

fn main() -> miette::Result<()> {
    // Ensure cargo rebuilds if this build script changes
    println!("cargo:rerun-if-changed=build.rs");

    // Build and link libSBML
    let sbml_build = build_and_link("vendors/libsbml", "sbml", true, false)?;

    // Configure autocxx to generate Rust bindings
    let rs_file = "src/lib.rs";
    // Point to the libSBML headers
    let sbml_include = format!("{}/include", sbml_build);
    let lib_root = ".";

    // Build the C++ wrapper code and bindings
    let mut b = autocxx_build::Builder::new(rs_file, &[lib_root, &sbml_include]).build()?;
    // Ensure C++17 is used for compilation
    b.flag_if_supported("-std=c++17").compile("libsbml");

    Ok(())
}

/// Helper function to build and link a C++ library using CMake
///
/// # Arguments
/// * `path` - Path to the library source directory
/// * `lib_name` - Name of the library to link against
///
/// # Returns
/// * The build directory path as a String
fn build_and_link(
    path: &str,
    lib_name: &str,
    build_shared: bool,
    static_lib: bool,
) -> miette::Result<String> {
    // Configure and build the library using CMake
    let build_shared_str = if build_shared { "ON" } else { "OFF" };
    let dst = cmake::Config::new(path)
        // Build shared libraries instead of static
        .define("BUILD_SHARED_LIBS", build_shared_str)
        // Skip configure step if CMake cache exists
        .always_configure(false)
        .build();

    // Configure cargo to link against the built library
    println!("cargo:rustc-link-search={}/lib", dst.display());

    if static_lib {
        println!("cargo:rustc-link-lib=static={}", lib_name);
    } else {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    Ok(dst.display().to_string())
}
