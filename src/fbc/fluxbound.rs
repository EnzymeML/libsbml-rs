//! This module provides a safe Rust interface to the libSBML FluxBound class.
//!
//! The FluxBound class represents a flux bound constraint in an SBML FBC (Flux Balance Constraints) model.
//! Flux bounds define constraints on the flux through reactions, specifying upper and lower bounds
//! using operations like less than, greater than, equal to, etc.
//!
//! This wrapper provides safe access to the underlying C++ libSBML FluxBound class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin};

use cxx::let_cxx_string;

use crate::{
    clone,
    errors::LibSBMLError,
    inner,
    model::Model,
    optional_property, pin_ptr,
    plugin::get_plugin,
    required_property, sbmlcxx,
    traits::{fromptr::FromPtr, intoid::IntoId},
    upcast_annotation,
};

use super::fluxboundop::FluxBoundOperation;

/// A safe wrapper around the libSBML FluxBound class.
///
/// FluxBound represents a constraint on the flux through a reaction in an SBML FBC model.
/// It consists of:
/// - An identifier (optional)
/// - A reaction identifier that this bound applies to
/// - An operation (less than, greater than, equal to, etc.)
/// - A value (handled separately in the FBC plugin)
///
/// This struct maintains a reference to the underlying C++ FluxBound object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct FluxBound<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::FluxBound>>,
}

inner!(sbmlcxx::FluxBound, FluxBound<'a>);

upcast_annotation!(FluxBound<'a>, sbmlcxx::FluxBound, sbmlcxx::SBase);

clone!(FluxBound<'a>, sbmlcxx::FluxBound);

impl<'a> FluxBound<'a> {
    /// Creates a new FluxBound instance within the given Model.
    ///
    /// This method creates a flux bound constraint that applies to a specific reaction
    /// with a given operation type (e.g., less than, greater than, equal to).
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this flux bound
    /// * `id` - The identifier for this flux bound (must be unique within the model)
    /// * `reaction_id` - The identifier for the reaction that this flux bound constrains
    /// * `operation` - The type of constraint operation (e.g., FluxBoundOperation::Less)
    ///
    /// # Returns
    /// A new FluxBound instance initialized with the given parameters and added to the model
    ///
    /// # Errors
    /// Returns `LibSBMLError` if:
    /// - The FBC plugin is not available or enabled in the model
    /// - The flux bound creation fails in the underlying libSBML library
    pub fn new(
        model: &Model<'a>,
        id: &str,
        reaction_id: impl IntoId,
        operation: impl Into<FluxBoundOperation>,
    ) -> Result<Self, LibSBMLError> {
        let mut fbc_plugin =
            get_plugin::<sbmlcxx::FbcModelPlugin, Model<'a>, sbmlcxx::Model>(model, "fbc")?;

        let flux_bound_ptr = fbc_plugin.as_mut().createFluxBound();
        let mut flux_bound = pin_ptr!(flux_bound_ptr, sbmlcxx::FluxBound);

        // Set the id of the flux bound
        let_cxx_string!(id = id);
        flux_bound.as_mut().setId(&id);

        // Set the reaction of the flux bound
        let_cxx_string!(reaction_id = reaction_id.into_id());
        flux_bound.as_mut().setReaction(&reaction_id);

        // Set the operation of the flux bound
        let operation = operation.into();
        flux_bound.as_mut().setOperation1(operation.into());

        Ok(Self {
            inner: RefCell::new(flux_bound),
        })
    }

    // Getter and setter for id
    optional_property!(FluxBound<'a>, id, String, getId, setId, isSetId);

    // Getter and setter for reaction
    optional_property!(
        FluxBound<'a>,
        reaction,
        String,
        getReaction,
        setReaction,
        isSetReaction
    );

    // Getter and setter for operation
    required_property!(
        FluxBound<'a>,
        operation,
        FluxBoundOperation,
        getFluxBoundOperation,
        setOperation1
    );
}

impl<'a> FromPtr<sbmlcxx::FluxBound> for FluxBound<'a> {
    fn from_ptr(ptr: *mut sbmlcxx::FluxBound) -> Self {
        let flux_bound = pin_ptr!(ptr, sbmlcxx::FluxBound);

        Self {
            inner: RefCell::new(flux_bound),
        }
    }
}

impl<'a> std::fmt::Debug for FluxBound<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("FluxBound");
        ds.field("id", &self.id());
        ds.field("reaction", &self.reaction());
        ds.field("operation", &self.operation());
        ds.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::Model, sbmldoc::SBMLDocument};

    #[test]
    fn test_flux_bound_new() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let flux_bound = FluxBound::new(&model, "fb1", "reaction1", FluxBoundOperation::LessEqual)
            .expect("Failed to create flux bound");

        assert_eq!(flux_bound.id(), Some("fb1".to_string()));
        assert_eq!(flux_bound.reaction(), Some("reaction1".to_string()));
        assert_eq!(flux_bound.operation(), FluxBoundOperation::LessEqual);
    }

    #[test]
    fn test_flux_bound_new_with_different_operations() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let operations = [
            FluxBoundOperation::Less,
            FluxBoundOperation::Greater,
            FluxBoundOperation::LessEqual,
            FluxBoundOperation::GreaterEqual,
            FluxBoundOperation::Equal,
        ];

        for (i, operation) in operations.iter().enumerate() {
            let id = format!("fb{i}");
            let reaction_id = format!("reaction{i}");

            let flux_bound = FluxBound::new(&model, &id, &reaction_id, *operation)
                .expect("Failed to create flux bound");

            assert_eq!(flux_bound.id(), Some(id));
            assert_eq!(flux_bound.reaction(), Some(reaction_id));
            assert_eq!(flux_bound.operation(), *operation);
        }
    }

    #[test]
    fn test_flux_bound_id_operations() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let flux_bound =
            FluxBound::new(&model, "original_id", "reaction1", FluxBoundOperation::Less)
                .expect("Failed to create flux bound");

        // Test initial ID
        assert_eq!(flux_bound.id(), Some("original_id".to_string()));

        // Test setting new ID
        flux_bound.set_id("new_id");
        assert_eq!(flux_bound.id(), Some("new_id".to_string()));
    }

    #[test]
    fn test_flux_bound_reaction_operations() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let flux_bound = FluxBound::new(
            &model,
            "fb1",
            "original_reaction",
            FluxBoundOperation::Greater,
        )
        .expect("Failed to create flux bound");

        // Test initial reaction
        assert_eq!(flux_bound.reaction(), Some("original_reaction".to_string()));

        // Test setting new reaction
        flux_bound.set_reaction("new_reaction");
        assert_eq!(flux_bound.reaction(), Some("new_reaction".to_string()));
    }

    #[test]
    fn test_flux_bound_operation_operations() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let flux_bound = FluxBound::new(&model, "fb1", "reaction1", FluxBoundOperation::Less)
            .expect("Failed to create flux bound");

        // Test initial operation
        assert_eq!(flux_bound.operation(), FluxBoundOperation::Less);

        // Test setting new operation
        flux_bound.set_operation(FluxBoundOperation::GreaterEqual);
        assert_eq!(flux_bound.operation(), FluxBoundOperation::GreaterEqual);

        // Test setting operation with Into trait
        flux_bound.set_operation(FluxBoundOperation::Equal);
        assert_eq!(flux_bound.operation(), FluxBoundOperation::Equal);
    }

    #[test]
    fn test_flux_bound_from_ptr() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        // Create a flux bound through the model to get a pointer
        let flux_bound1 = FluxBound::new(&model, "fb1", "reaction1", FluxBoundOperation::LessEqual)
            .expect("Failed to create flux bound");

        // Test that the flux bound was created correctly
        assert_eq!(flux_bound1.id(), Some("fb1".to_string()));
        assert_eq!(flux_bound1.reaction(), Some("reaction1".to_string()));
        assert_eq!(flux_bound1.operation(), FluxBoundOperation::LessEqual);
    }

    #[test]
    fn test_flux_bound_debug() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let flux_bound = FluxBound::new(
            &model,
            "debug_test",
            "debug_reaction",
            FluxBoundOperation::Equal,
        )
        .expect("Failed to create flux bound");

        let debug_string = format!("{flux_bound:?}");
        assert!(debug_string.contains("FluxBound"));
        assert!(debug_string.contains("debug_test"));
        assert!(debug_string.contains("debug_reaction"));
        assert!(debug_string.contains("Equal"));
    }

    #[test]
    fn test_flux_bound_with_string_reaction_id() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        // Test with String
        let reaction_id = String::from("string_reaction");
        let flux_bound = FluxBound::new(&model, "fb1", reaction_id, FluxBoundOperation::Greater)
            .expect("Failed to create flux bound");

        assert_eq!(flux_bound.reaction(), Some("string_reaction".to_string()));
    }

    #[test]
    fn test_flux_bound_with_str_reaction_id() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        // Test with &str
        let flux_bound =
            FluxBound::new(&model, "fb1", "str_reaction", FluxBoundOperation::LessEqual)
                .expect("Failed to create flux bound");

        assert_eq!(flux_bound.reaction(), Some("str_reaction".to_string()));
    }

    #[test]
    fn test_flux_bound_clone() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let flux_bound = FluxBound::new(&model, "fb1", "reaction1", FluxBoundOperation::Less)
            .expect("Failed to create flux bound");

        // Test that clone works (this tests the clone! macro)
        let cloned = flux_bound.clone();
        assert_eq!(cloned.id(), flux_bound.id());
        assert_eq!(cloned.reaction(), flux_bound.reaction());
        assert_eq!(cloned.operation(), flux_bound.operation());
    }

    #[test]
    fn test_flux_bound_comprehensive_workflow() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "comprehensive_test");

        // Create multiple flux bounds with different properties
        let fb1 = FluxBound::new(
            &model,
            "lower_bound",
            "glycolysis_reaction",
            FluxBoundOperation::GreaterEqual,
        )
        .expect("Failed to create lower bound");

        let fb2 = FluxBound::new(
            &model,
            "upper_bound",
            "glycolysis_reaction",
            FluxBoundOperation::LessEqual,
        )
        .expect("Failed to create upper bound");

        let fb3 = FluxBound::new(
            &model,
            "exact_bound",
            "transport_reaction",
            FluxBoundOperation::Equal,
        )
        .expect("Failed to create exact bound");

        // Verify initial state
        assert_eq!(fb1.id(), Some("lower_bound".to_string()));
        assert_eq!(fb1.reaction(), Some("glycolysis_reaction".to_string()));
        assert_eq!(fb1.operation(), FluxBoundOperation::GreaterEqual);

        assert_eq!(fb2.id(), Some("upper_bound".to_string()));
        assert_eq!(fb2.reaction(), Some("glycolysis_reaction".to_string()));
        assert_eq!(fb2.operation(), FluxBoundOperation::LessEqual);

        assert_eq!(fb3.id(), Some("exact_bound".to_string()));
        assert_eq!(fb3.reaction(), Some("transport_reaction".to_string()));
        assert_eq!(fb3.operation(), FluxBoundOperation::Equal);

        // Modify properties
        fb1.set_id("modified_lower_bound");
        fb1.set_reaction("modified_reaction");
        fb1.set_operation(FluxBoundOperation::Greater);

        // Verify modifications
        assert_eq!(fb1.id(), Some("modified_lower_bound".to_string()));
        assert_eq!(fb1.reaction(), Some("modified_reaction".to_string()));
        assert_eq!(fb1.operation(), FluxBoundOperation::Greater);

        // Ensure other flux bounds are unchanged
        assert_eq!(fb2.id(), Some("upper_bound".to_string()));
        assert_eq!(fb3.id(), Some("exact_bound".to_string()));
    }
}
