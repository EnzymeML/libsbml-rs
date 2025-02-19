fn main() -> miette::Result<()> {
    // Rebuild if these files change.
    println!("cargo:rerun-if-changed=build.rs");

    // Build the libsbml submodule using cmake
    let dst = cmake::Config::new("libsbml")
        .always_configure(false) // Only reconfigure when files change.
        .build();

    // Add the built library to linker search path
    println!("cargo:rustc-link-search={}/lib", dst.display());

    // Link against the actual libSBML library.
    println!("cargo:rustc-link-lib=sbml");

    // Build the autocxx generated wrapper.
    let rs_file = "src/lib.rs";
    let sbml_include = format!("{}/include", dst.display());
    let utils_hpp = format!("{}/utils.hpp", sbml_include);
    let lib_root = ".";

    let mut b =
        autocxx_build::Builder::new(rs_file, &[lib_root, &sbml_include, &utils_hpp]).build()?;
    b.flag_if_supported("-std=c++17").compile("libsbml");

    Ok(())
}
