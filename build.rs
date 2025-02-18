fn main() -> miette::Result<()> {
    // Set the link search path to where libSBML is installed.
    println!("cargo:rustc-link-search=/opt/homebrew/Cellar/libsbml/5.20.4/lib");

    // Link against the actual libSBML library.
    println!("cargo:rustc-link-lib=sbml");

    // Rebuild if these files change.
    println!("cargo:rerun-if-changed=build.rs");

    // Build the autocxx generated wrapper.
    let mut b = autocxx_build::Builder::new(
        "src/lib.rs",
        &[
            ".",
            "/opt/homebrew/Cellar/libsbml/5.20.4/include/",
            "src/annotation.hpp",
        ],
    )
    .build()?;
    b.flag_if_supported("-std=c++17").compile("libsbml");

    Ok(())
}
