<div align="center">

# 🧬 Rust SBML

A Rust crate providing a robust interface for reading and writing SBML (Systems Biology Markup Language) files.
Built as an ergonomic wrapper around the `libsbml` C++ library with type-safe Rust abstractions.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Windows%20%7C%20Linux-blue)]()

</div>

## 🚀 Features

- Type-safe builder pattern API for SBML model creation
- Seamless serialization/deserialization of annotations using `serde`
- Bundled `libsbml` library - no external installation required
- Cross-platform support (macOS and Windows)
- Comprehensive error handling and type safety

## 📦 Installation

Currently available through Git:

```bash
cargo add --git https://github.com/EnzymeML/sbml-rs
```

### System Requirements

- **CMake**: Required for building the bundled `libsbml` library
- **Rust**: 1.70 or higher (recommended)

### Platform Support

✅ Tested and supported:

- macOS (arm64, x86_64)
- Windows (x86_64)
- Linux (x86_64)

Please note, due to ongoing development to support platforms, the crate is currently only available as a static library for Windows and Linux. MacOS is available as a dynamic library. We will update this section as we add more support.

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

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

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
| KineticLaw               | ❌ Not yet implemented |
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
| LocalParameter           | ❌ Not yet implemented |
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
2. Add support for SBML Level 3 packages (Flux Balance Constraints, Hierarchical Model Composition, etc.)
3. Improve error handling and validation
4. Add more examples and documentation
5. Performance optimizations

