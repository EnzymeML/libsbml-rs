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
//! - Compartment: Represents physical containers/spaces in the model
//! - Reaction: Represents biochemical reactions between species
//! - Parameter: Represents numerical parameters used in the model
//! - Unit/UnitDefinition: Represents units of measurement
//! - SpeciesReference: Represents species participating in reactions

/// Module providing traits for the SBML library
pub mod traits {
    pub mod annotation;
    pub mod fromptr;
    pub mod inner;
    pub mod intoid;
}

/// Module providing upcast functionality
pub mod cast;
/// Module providing compartment functionality
pub mod compartment;
/// Module providing model functionality
pub mod model;
/// Module providing modifier species reference functionality
pub mod modref;
/// Module providing parameter functionality
pub mod parameter;
/// Module providing reaction functionality
pub mod reaction;
/// Module providing rate rule functionality
pub mod rule;
/// Module providing core SBML document functionality
pub mod sbmldoc;
/// Module providing species functionality
pub mod species;
/// Module providing species reference functionality
pub mod speciesref;
/// Module providing unit functionality
pub mod unit;
/// Module providing unit definition functionality
pub mod unitdef;

/// Module containing helper macros
pub mod macros;

/// Module containing reader functionality
pub mod reader;

/// Internal module containing the wrapper types for the annotation.
pub(crate) mod wrapper;

/// Internal module containing the container types for the annotation. This is mainly used to extract annotations from the model. Collections are handled by the [`Model`] struct.
pub(crate) mod collections {
    pub(crate) use crate::collections::compartments::*;
    pub(crate) use crate::collections::parameters::*;
    pub(crate) use crate::collections::reactions::*;
    pub(crate) use crate::collections::rules::*;
    pub(crate) use crate::collections::species::*;
    pub(crate) use crate::collections::unitdefs::*;

    pub(crate) mod compartments;
    pub(crate) mod parameters;
    pub(crate) mod reactions;
    pub(crate) mod rules;
    pub(crate) mod species;
    pub(crate) mod unitdefs;
}

// Re-export commonly used types
pub use sbmldoc::SBMLDocument;
pub use traits::annotation::Annotation;

/// Prelude module providing convenient imports of commonly used types
pub mod prelude {
    pub use crate::compartment::Compartment;
    pub use crate::model::*;
    pub use crate::modref::*;
    pub use crate::parameter::*;
    pub use crate::reaction::*;
    pub use crate::reader::*;
    pub use crate::rule::*;
    pub use crate::sbmldoc::*;
    pub use crate::species::*;
    pub use crate::speciesref::*;
    pub use crate::traits::annotation::*;
    pub use crate::traits::intoid::*;
    pub use crate::unit::*;
    pub use crate::unitdef::*;
}

/// Internal module containing the raw FFI bindings to libSBML.
///
/// This module uses autocxx to generate safe Rust bindings to the C++ libSBML classes.
/// It is not intended to be used directly - instead use the safe wrapper types
/// provided in the other modules.
pub(crate) mod sbmlcxx {
    use autocxx::prelude::*;

    include_cpp! {
        // Includes //
        #include "sbml/SBMLTypes.h"
        safety!(unsafe_ffi)

        // Base types
        generate!("SBase")

        // Root types
        generate!("SBMLDocument")
        generate!("Model")

        // Leaf types
        generate!("Species")
        generate!("Parameter")
        generate!("Compartment")
        generate!("UnitDefinition")
        generate!("Unit")
        generate!("UnitKind_t")
        generate!("Reaction")
        generate!("SpeciesReference")
        generate!("SimpleSpeciesReference")
        generate!("ModifierSpeciesReference")
        generate!("InitialAssignment")
        generate!("RateRule")
        generate!("AssignmentRule")
        generate!("Rule")

        // IO types
        generate!("SBMLWriter")
        generate!("SBMLReader")

        // Container types
        generate!("ListOfParameters")
        generate!("ListOfUnitDefinitions")
        generate!("ListOfCompartments")
        generate!("ListOfSpecies")
        generate!("ListOfReactions")
        generate!("ListOfUnitDefinitions")
    }

    pub use ffi::*;
}
