//! # Rust bindings for libSBML - Systems Biology Markup Language library
//!
//! This crate provides safe, idiomatic Rust bindings to the C++ libSBML library for
//! working with SBML (Systems Biology Markup Language) files - a standard format for
//! representing computational models in systems biology research.
//!
//! ## Features
//!
//! - Complete access to libSBML functionality with a Rust-friendly API
//! - Type-safe interfaces for creating and manipulating SBML models
//! - Reading and writing SBML files with validation support
//! - Memory-safe wrappers around the C++ library using autocxx
//!
//! ## Base Types
//!
//! - **SBase** (`sbase`): Base class for all SBML components
//! - **SBasePlugin** (`sbaseplugin`): Plugin for SBase
//! - **SBMLNamespaces** (`namespaces`): Namespaces for SBML models
//!
//! ## Core Components
//!
//! - **SBMLDocument** (`sbmldoc`): Root container for SBML models
//! - **Model** (`model`): Biological model with species, reactions, and other components
//! - **Species** (`species`): Chemical entities/molecules in the model
//! - **Compartment** (`compartment`): Physical containers/spaces where species reside
//! - **Reaction** (`reaction`): Biochemical transformations with reactants, products and kinetics
//! - **Parameter** (`parameter`): Numerical values used throughout the model
//! - **LocalParameter** (`localparameter`): Parameters scoped to specific reactions
//! - **KineticLaw** (`kineticlaw`): Mathematical expressions defining reaction rates
//! - **Unit** (`unit`): Base units for quantities in the model
//! - **UnitDefinition** (`unitdef`): Composite units of measurement
//! - **Rule** (`rule`): Mathematical expressions that define model behavior
//! - **SpeciesReference** (`speciesref`): References to species as reactants or products
//! - **ModifierSpeciesReference** (`modref`): Species references for catalysts and regulators
//! - **FluxObjective** (`fluxobjective`): Objectives for flux balance analysis
//!
//! ## FBC Package
//!
//! - **Objective** (`objective`): Objectives for optimization
//! - **ListOfObjectives** (`listofobjectives`): List of objectives
//! - **ListOfFluxObjectives** (`listoffluxobjectives`): List of flux objectives
//! - **FluxObjective** (`fluxobjective`): Objectives for flux balance analysis

/// Traits providing common functionality across SBML components
pub mod traits {
    pub mod annotation;
    pub mod fromptr;
    pub mod inner;
    pub mod intoid;
    pub mod sbase;
}

/// Type casting and conversion utilities for SBML objects
pub mod cast;
/// Compartments representing physical containers in the model
pub mod compartment;
/// Kinetic laws that define reaction rates and mathematics
pub mod kineticlaw;
/// Local parameters scoped to specific reactions or expressions
pub mod localparameter;
/// Model definition and management for biological systems
pub mod model;
/// Modifier species references for catalysts and regulators
pub mod modref;
/// Namespaces for SBML models
pub mod namespaces;
/// Global parameters defining constant or variable model values
pub mod parameter;
/// Reactions describing biochemical transformations between species
pub mod reaction;
/// Rules for mathematical constraints and assignments within models
pub mod rule;
/// Core document handling for SBML files and model containers
pub mod sbmldoc;
/// Species representing chemical entities and molecules
pub mod species;
/// References to species as reactants or products in reactions
pub mod speciesref;
/// Units of measurement for model quantities
pub mod unit;
/// Unit definitions composing multiple base units
pub mod unitdef;

/// Packages for SBML models
pub mod packages;
/// Plugin fetcher
pub mod plugin;
/// Error handling for SBML models
pub mod sbmlerror;

/// FBC package types
pub mod fbc {
    pub use crate::fbc::fluxbound::FluxBound;
    pub use crate::fbc::fluxboundop::FluxBoundOperation;
    pub use crate::fbc::objective::Objective;
    pub use crate::fbc::objectivetype::ObjectiveType;

    /// Flux bound
    pub mod fluxbound;
    /// Flux bound operation types
    pub mod fluxboundop;
    /// A flux objective
    pub mod fluxobjective;
    /// A general objective
    pub mod objective;
    /// Objective types
    pub mod objectivetype;
}

/// Helper macros for working with SBML components
pub mod macros;

/// Property handling for SBML element attributes
pub mod property;

/// File and model input/output operations
pub mod reader;

/// Internal module containing the wrapper types for annotations
pub(crate) mod wrapper;

/// Error handling for SBML models
pub mod errors;

/// Internal module containing collections of SBML components
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
    pub use crate::fbc::*;
    pub use crate::kineticlaw::*;
    pub use crate::localparameter::*;
    pub use crate::model::*;
    pub use crate::modref::*;
    pub use crate::parameter::*;
    pub use crate::reaction::*;
    pub use crate::reader::*;
    pub use crate::rule::*;
    pub use crate::sbmldoc::*;
    pub use crate::sbmlerror::*;
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
        #include "sbml/packages/fbc/common/FbcExtensionTypes.h"
        safety!(unsafe_ffi)

        // Base types
        generate!("SBase")
        generate!("SBasePlugin")
        generate!("SBMLNamespaces")
        generate!("XMLNamespaces")

        // Root types
        generate!("SBMLDocument")
        generate!("Model")

        // Leaf types
        generate!("Species")
        generate!("Parameter")
        generate!("LocalParameter")
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
        generate!("KineticLaw")

        // FBC types
        generate!("FbcModelPlugin")
        generate!("ListOfFluxObjectives")
        generate!("FluxObjective")
        generate!("FluxBound")
        generate!("Objective")
        generate!("ListOfObjectives")
        generate!("ObjectiveType_t")
        generate!("FluxBoundOperation_t")

        // IO types
        generate!("SBMLWriter")
        generate!("SBMLReader")

        // Validation types
        generate!("SBMLValidator")
        generate!("SBMLInternalValidator")
        generate!("SBMLError")
        generate!("SBMLErrorLog")
        generate!("XMLError")

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
