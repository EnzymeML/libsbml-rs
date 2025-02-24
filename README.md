# Rust SBML

A Rust crate for reading and writing SBML files. This crate provides a thin wrapper around the `libsbml` C++ library and Rust structs to represent the SBML model.

## Installation

This crate is not yet published to crates.io, so you need to add it to your `Cargo.toml` via the git repository.

```bash
cargo add --git https://github.com/JR-1991/sbml-rs
```

### Dependencies

This crate bundles the `libsbml` library and its dependencies, so you don't need to install it separately. However, you need to have the following dependencies installed on your system:

- `CMake` - for building the `libsbml` library

In the future, the crate will also allow you to provide an already installed `libsbml` library to use.

### Platforms

Currently, the crate is tested for the following platforms:

- macOS (arm64, x86_64)
- Windows (x86_64)

At the moment, the crate is not running on Linux (x86_64), but will be made available soon.

## Usage

The crate builds upon the SBML workflow by cascading objects through the root `SBMLDocument` object. We provide both a setter- and a builder-style API to create SBML models. The builder style is recommended, as it is more type-safe and easier to read, but this is a matter of preference.

### Creation of SBML models

```rust
use sbml::prelude::*;

let doc = SBMLDocument::new(3, 2);

// Create a model
let model = doc.create_model("Model");

// Create a compartment
let compartment = model.build_compartment("cytosol")
    .name("Cytosol")
    .build();

// Create the glucose species with the annotation
let glucose = model
    .build_species("glucose")
    .name("Glucose")
    .compartment(&compartment.id())
    .initial_amount(10.0)
    .boundary_condition(true)
    .annotation_serde(&glucose_annotation)?
    .build();

// Serialize the document to an SBML string
let sbml_string = doc.to_xml_string();

// Print the SBML string
println!("{}", sbml_string);
```

### Adding annotations to SBML models

Annotations play a key role in SBML models, as they allow for a flexible extension of the SBML model. Since the C++ `libsbml` library either expects an annotation string or `XMLNode` object, which is not type-safe, we provide a `serde` implementation for the `Annotation` struct.

Hence, adding an annotation to an SBML model is as easy as:

```rust
use sbml::prelude::*;

// Deifnition of your annotation struct
#[derive(Serialize, Deserialize, Debug)]
struct MyAnnotation {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    key: String,
    value: i32,
}

let doc = SBMLDocument::new(3, 2);
let model = doc.create_model("Model");

// Create an annotation
let glucose_annotation = MyAnnotation {
    xmlns: "http://my.namespace.com".to_string(),
    key: "test".to_string(),
    value: 1,
};

let species = model.build_species("glucose")
    .name("Glucose")
    .compartment(&compartment.id())
    .initial_amount(10.0)
    .boundary_condition(true)
    .annotation_serde(&glucose_annotation)?
    .build();
```

In the same way, you can also read annotations from an SBML model, by using the `get_annotation_serde` method. Rust's type inference will then infer the correct type of the annotation and extract it into the correct struct.

```rust
let glucose_annotation: MyAnnotation = species.get_annotation_serde()?;
```

We refer to the [serde](https://serde.rs/) and [quick-xml](https://docs.rs/quick-xml/latest/quick_xml/) documentation for more information on how to use the `serde` and `quick-xml` crates for serializing and deserializing annotations.

## Acknowledgements

This crate is a Rust port of the [libsbml](https://github.com/sbmlteam/libsbml) library.

## License

This crate is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.