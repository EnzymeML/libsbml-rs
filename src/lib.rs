//! Rust bindings for libSBML - Systems Biology Markup Language library
//!
//! This crate provides safe Rust bindings to the C++ libSBML library, which is used for
//! reading, writing, manipulating and validating SBML (Systems Biology Markup Language) files.
//! SBML is a widely used format for representing computational models in systems biology.
//!
//! The bindings are generated using autocxx to provide a safe interface while maintaining
//! close integration with the underlying C++ library. The main components include:
//!
//! - SBMLDocument: The root container for SBML models
//! - Model: Represents a biological model with species, reactions etc.
//! - Species: Represents chemical species/entities in the model
//! - Parameter: Represents parameters used in the model
//!
//! # Example
//! ```
//! use sbml::SBMLDocument;
//!
//! let mut document = SBMLDocument::new(3, 2); // Create SBML L3V2 document
//! let model = document.create_model("example");
//! let species = model.create_species("glucose");
//! ```

pub mod annotation;
pub mod compartment;
pub mod model;
pub mod sbmldoc;
pub mod species;
pub mod unit;
pub mod unitdef;

/// Internal module containing the wrapper types for the annotation.
pub(crate) mod wrapper;

// Re-export commonly used types
pub use annotation::Annotation;
pub use sbmldoc::SBMLDocument;

pub mod prelude {
    pub use crate::annotation::Annotation;
    pub use crate::compartment::Compartment;
    pub use crate::model::Model;
    pub use crate::sbmldoc::SBMLDocument;
    pub use crate::species::{Species, SpeciesBuilder};
    pub use crate::unit::Unit;
    pub use crate::unitdef::UnitDefinition;
}

/// Internal module containing the raw FFI bindings to libSBML.
///
/// This module uses autocxx to generate safe Rust bindings to the C++ libSBML classes.
/// It is not intended to be used directly - instead use the safe wrapper types
/// provided in the other modules.
pub(crate) mod sbmlcxx {
    use autocxx::prelude::*;

    include_cpp! {
        #include "sbml/SBMLTypes.h"
        #include "src/utils.hpp"
        safety!(unsafe_ffi)
        // libsbml types
        generate!("SBase")
        generate!("Model")
        generate!("Species")
        generate!("Parameter")
        generate!("Compartment")
        generate!("SBMLDocument")
        generate!("SBMLWriter")
        generate!("UnitDefinition")
        generate!("Unit")
        generate!("UnitKind_t")
        // utils
        generate!("utils::getSpeciesAnnotationString")
        generate!("utils::getModelAnnotationString")
        generate!("utils::getCompartmentAnnotationString")
        generate!("utils::setSpeciesAnnotation")
        generate!("utils::setModelAnnotation")
        generate!("utils::setCompartmentAnnotation")
    }

    pub use ffi::*;
}
