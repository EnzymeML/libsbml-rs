<div align="center">

# 🧬 Rust SBML

A Rust crate providing a robust interface for reading and writing SBML (Systems Biology Markup Language) files.
Built as an ergonomic wrapper around the `libsbml` C++ library with type-safe Rust abstractions.

[![License: LGPL v3](https://img.shields.io/badge/License-LGPL%20v3-blue.svg)](LICENSE.txt)
[![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Windows%20%7C%20Linux-blue)]()

</div>

## 🚀 Features

- Type-safe builder pattern API for SBML model creation
- Seamless serialization/deserialization of annotations using `serde`
- Automatic C++ dependency management via `cargo-vcpkg`
- Cross-platform support (macOS, Windows, Linux)
- Comprehensive error handling and type safety

## 📦 Installation

Currently available through Git:

```bash
cargo add --git https://github.com/EnzymeML/sbml-rs
```

Please note, the C++ dependency `libsbml` is automatically installed using `cargo-vcpkg`. You dont need to link the library manually, but note that the `build.rs` script will install `cargo-vcpkg` if it is not found in your environment.

### System Requirements

- **Rust**: 1.70 or higher (recommended)

### Platform Support

✅ Tested and supported:

- macOS (arm64, x86_64)
- Windows (x86_64)
- Linux (x86_64)

## 💡 Usage

The crate follows SBML's hierarchical structure, with all operations flowing through the root `SBMLDocument`. We offer two API styles:

1. Builder pattern (recommended)
2. Traditional setter methods

### Creating SBML Models

```rust
use sbml::prelude::*;

let doc = SBMLDocument::new(3, 2);

// Create a model
let model = doc.create_model("Model");

// Create a compartment
let compartment = model.build_compartment("cytosol")
    .name("Cytosol")
    .build();

// Create the glucose species with annotation
let glucose = model
    .build_species("glucose")
    .name("Glucose")
    .compartment(&compartment.id())
    .initial_amount(10.0)
    .boundary_condition(true)
    .annotation_serde(&glucose_annotation)?
    .build();

// Export to SBML XML
let sbml_string = doc.to_xml_string();
```

### Type-Safe Annotations

Leverage Rust's type system for SBML annotations:

```rust
use sbml::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct MyAnnotation {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    key: String,
    value: i32,
}

// Create and attach annotation
let annotation = MyAnnotation {
    xmlns: "http://my.namespace.com".to_string(),
    key: "test".to_string(),
    value: 1,
};

let species = model.build_species("glucose")
    .name("Glucose")
    .compartment(&compartment.id())
    .initial_amount(10.0)
    .boundary_condition(true)
    .annotation_serde(&annotation)?
    .build();

// Read annotation
let retrieved: MyAnnotation = species.get_annotation_serde()?;
```

## 🛠️ Development

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features

# Run tests
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📚 Resources

- [SBML Documentation](http://sbml.org/Documents/Specifications)
- [libsbml Documentation](http://sbml.org/Software/libSBML)

## 🙏 Acknowledgements

This crate is a Rust port of the [libsbml](https://github.com/sbmlteam/libsbml) library. Special thanks to the SBML team for their excellent work.

## 📄 License

This project is licensed under the LGPL License - see the [LICENSE](LICENSE.txt) file for details.

## 🚧 Roadmap

The following table shows the current implementation status of SBML objects in this crate:

| SBML Object              | Status                |
| ------------------------ | --------------------- |
| Model                    | ⚠️ Partially           |
| Compartment              | ✅ Implemented         |
| Species                  | ✅ Implemented         |
| Parameter                | ✅ Implemented         |
| Reaction                 | ✅ Implemented         |
| Rule                     | ✅ Implemented         |
| AssignmentRule           | ✅ Implemented         |
| RateRule                 | ✅ Implemented         |
| UnitDefinition           | ✅ Implemented         |
| Unit                     | ✅ Implemented         |
| KineticLaw               | ✅ Implemented         |
| SpeciesReference         | ✅ Implemented         |
| ModifierSpeciesReference | ✅ Implemented         |
| InitialAssignment        | ❌ Not yet implemented |
| Event                    | ❌ Not yet implemented |
| EventAssignment          | ❌ Not yet implemented |
| Trigger                  | ❌ Not yet implemented |
| Delay                    | ❌ Not yet implemented |
| Priority                 | ❌ Not yet implemented |
| FunctionDefinition       | ❌ Not yet implemented |
| Constraint               | ❌ Not yet implemented |
| LocalParameter           | ✅ Implemented         |
| StoichiometryMath        | ❌ Not yet implemented |
| CompartmentType          | ❌ Not yet implemented |
| SpeciesType              | ❌ Not yet implemented |
| SBase                    | ✅ Implemented         |
| ListOf                   | ✅ Implemented         |
| ASTNode                  | ❌ Not yet implemented |
| CVTerm                   | ❌ Not yet implemented |
| Date                     | ❌ Not yet implemented |
| ModelHistory             | ❌ Not yet implemented |
| ModelCreator             | ❌ Not yet implemented |

Future development priorities:

1. Complete implementation of remaining SBML core objects
2. Improve error handling and validation
3. Add more examples and documentation
4. Performance optimizations

