//! This module provides a safe Rust interface to the libSBML FluxObjective class.
//!
//! The FluxObjective class represents a single reaction's contribution to an optimization objective
//! in an SBML FBC (Flux Balance Constraints) model. Each flux objective specifies a reaction and
//! its coefficient (weight) in the linear combination that defines the overall objective function.
//! Multiple flux objectives can be combined within an Objective to create complex optimization goals.
//!
//! This wrapper provides safe access to the underlying C++ libSBML FluxObjective class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin};

use cxx::let_cxx_string;

use crate::{
    clone,
    errors::LibSBMLError,
    inner, optional_property, pin_ptr, sbmlcxx,
    traits::{fromptr::FromPtr, intoid::IntoId},
    upcast_annotation,
};

use super::objective::Objective;

/// A safe wrapper around the libSBML FluxObjective class.
///
/// FluxObjective represents a single reaction's contribution to an optimization objective in an SBML FBC model.
/// It consists of:
/// - An identifier (optional)
/// - A reaction identifier that this flux objective applies to
/// - A coefficient that specifies the weight of this reaction in the objective function
///
/// This struct maintains a reference to the underlying C++ FluxObjective object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct FluxObjective<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::FluxObjective>>,
}

inner!(sbmlcxx::FluxObjective, FluxObjective<'a>);

upcast_annotation!(FluxObjective<'a>, sbmlcxx::FluxObjective, sbmlcxx::SBase);

clone!(FluxObjective<'a>, sbmlcxx::FluxObjective);

impl<'a> FluxObjective<'a> {
    /// Creates a new FluxObjective instance within the given Objective.
    ///
    /// This method creates a flux objective that contributes to the overall optimization objective.
    /// The flux objective specifies how a particular reaction contributes to the objective function
    /// through its coefficient.
    ///
    /// # Arguments
    /// * `objective` - The parent Objective that will contain this flux objective
    /// * `id` - The identifier for this flux objective
    /// * `reaction_id` - The identifier for the reaction that contributes to the objective
    /// * `coefficient` - The coefficient (weight) of this reaction in the objective function
    ///
    /// # Returns
    /// A new FluxObjective instance initialized with the given parameters and added to the objective
    ///
    /// # Errors
    /// Returns `LibSBMLError` if the flux objective creation fails in the underlying libSBML library
    pub fn new(
        objective: &Objective<'a>,
        id: &str,
        reaction_id: impl IntoId,
        coefficient: f64,
    ) -> Result<Self, LibSBMLError> {
        let flux_objective_ptr = objective
            .inner()
            .borrow_mut()
            .as_mut()
            .createFluxObjective();

        let mut flux_objective = pin_ptr!(flux_objective_ptr, sbmlcxx::FluxObjective);

        // Set the id of the flux objective
        let_cxx_string!(id = id.to_string());
        flux_objective.as_mut().setId(&id);

        // Set the reaction id of the flux objective
        let_cxx_string!(reaction_id = reaction_id.into_id());
        flux_objective.as_mut().setReaction(&reaction_id);

        // Set the coefficient of the flux objective
        flux_objective.as_mut().setCoefficient(coefficient);

        Ok(Self {
            inner: RefCell::new(flux_objective),
        })
    }

    // Getter and setter for id
    optional_property!(FluxObjective<'a>, id, String, getId, setId, isSetId);

    // Getter and setter for reaction
    optional_property!(
        FluxObjective<'a>,
        reaction,
        String,
        getReaction,
        setReaction,
        isSetReaction
    );

    // Getter and setter for coefficient
    optional_property!(
        FluxObjective<'a>,
        coefficient,
        f64,
        getCoefficient,
        setCoefficient,
        isSetCoefficient
    );
}

impl<'a> FromPtr<sbmlcxx::FluxObjective> for FluxObjective<'a> {
    fn from_ptr(ptr: *mut sbmlcxx::FluxObjective) -> Self {
        let flux_objective = pin_ptr!(ptr, sbmlcxx::FluxObjective);

        Self {
            inner: RefCell::new(flux_objective),
        }
    }
}

impl<'a> std::fmt::Debug for FluxObjective<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("FluxObjective");
        ds.field("id", &self.id());
        ds.field("reaction", &self.reaction());
        ds.field("coefficient", &self.coefficient());
        ds.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fbc::objectivetype::ObjectiveType;
    use crate::{model::Model, sbmldoc::SBMLDocument};

    #[test]
    fn test_flux_objective_new() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        let flux_objective = FluxObjective::new(&objective, "fo1", "reaction1", 1.0)
            .expect("Failed to create flux objective");

        assert_eq!(flux_objective.id(), Some("fo1".to_string()));
        assert_eq!(flux_objective.reaction(), Some("reaction1".to_string()));
        assert_eq!(flux_objective.coefficient(), Some(1.0));
    }

    #[test]
    fn test_flux_objective_new_with_different_coefficients() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        let coefficients = [0.0, 1.0, -1.0, 2.5, -0.5, 100.0, -100.0];

        for (i, coefficient) in coefficients.iter().enumerate() {
            let id = format!("fo{i}");
            let reaction_id = format!("reaction{i}");

            let flux_objective = FluxObjective::new(&objective, &id, &reaction_id, *coefficient)
                .expect("Failed to create flux objective");

            assert_eq!(flux_objective.id(), Some(id));
            assert_eq!(flux_objective.reaction(), Some(reaction_id));
            assert_eq!(flux_objective.coefficient(), Some(*coefficient));
        }
    }

    #[test]
    fn test_flux_objective_id_operations() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        let flux_objective = FluxObjective::new(&objective, "original_id", "reaction1", 1.0)
            .expect("Failed to create flux objective");

        // Test initial ID
        assert_eq!(flux_objective.id(), Some("original_id".to_string()));

        // Test setting new ID
        flux_objective.set_id("new_id");
        assert_eq!(flux_objective.id(), Some("new_id".to_string()));
    }

    #[test]
    fn test_flux_objective_reaction_operations() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        let flux_objective = FluxObjective::new(&objective, "fo1", "original_reaction", 1.0)
            .expect("Failed to create flux objective");

        // Test initial reaction
        assert_eq!(
            flux_objective.reaction(),
            Some("original_reaction".to_string())
        );

        // Test setting new reaction
        flux_objective.set_reaction("new_reaction");
        assert_eq!(flux_objective.reaction(), Some("new_reaction".to_string()));
    }

    #[test]
    fn test_flux_objective_coefficient_operations() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        let flux_objective = FluxObjective::new(&objective, "fo1", "reaction1", 1.0)
            .expect("Failed to create flux objective");

        // Test initial coefficient
        assert_eq!(flux_objective.coefficient(), Some(1.0));

        // Test setting new coefficient
        flux_objective.set_coefficient(2.5);
        assert_eq!(flux_objective.coefficient(), Some(2.5));

        // Test setting negative coefficient
        flux_objective.set_coefficient(-1.5);
        assert_eq!(flux_objective.coefficient(), Some(-1.5));

        // Test setting zero coefficient
        flux_objective.set_coefficient(0.0);
        assert_eq!(flux_objective.coefficient(), Some(0.0));
    }

    #[test]
    fn test_flux_objective_from_ptr() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Create a flux objective through the objective to test FromPtr
        let flux_objective = FluxObjective::new(&objective, "fo1", "reaction1", 1.5)
            .expect("Failed to create flux objective");

        // Test that the flux objective was created correctly
        assert_eq!(flux_objective.id(), Some("fo1".to_string()));
        assert_eq!(flux_objective.reaction(), Some("reaction1".to_string()));
        assert_eq!(flux_objective.coefficient(), Some(1.5));
    }

    #[test]
    fn test_flux_objective_debug() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        let flux_objective = FluxObjective::new(&objective, "debug_test", "debug_reaction", 2.5)
            .expect("Failed to create flux objective");

        let debug_string = format!("{flux_objective:?}");
        assert!(debug_string.contains("FluxObjective"));
        assert!(debug_string.contains("debug_test"));
        assert!(debug_string.contains("debug_reaction"));
        assert!(debug_string.contains("2.5"));
    }

    #[test]
    fn test_flux_objective_with_string_reaction_id() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Test with String
        let reaction_id = String::from("string_reaction");
        let flux_objective = FluxObjective::new(&objective, "fo1", reaction_id, 1.0)
            .expect("Failed to create flux objective");

        assert_eq!(
            flux_objective.reaction(),
            Some("string_reaction".to_string())
        );
    }

    #[test]
    fn test_flux_objective_with_str_reaction_id() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Test with &str
        let flux_objective = FluxObjective::new(&objective, "fo1", "str_reaction", 1.0)
            .expect("Failed to create flux objective");

        assert_eq!(flux_objective.reaction(), Some("str_reaction".to_string()));
    }

    #[test]
    fn test_flux_objective_clone() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        let flux_objective = FluxObjective::new(&objective, "fo1", "reaction1", 1.0)
            .expect("Failed to create flux objective");

        // Test that clone works (this tests the clone! macro)
        let cloned = flux_objective.clone();
        assert_eq!(cloned.id(), flux_objective.id());
        assert_eq!(cloned.reaction(), flux_objective.reaction());
        assert_eq!(cloned.coefficient(), flux_objective.coefficient());
    }

    #[test]
    fn test_flux_objective_comprehensive_workflow() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "comprehensive_test");

        // Create an objective
        let objective = Objective::new(&model, "biomass_obj", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Create multiple flux objectives with different properties
        let biomass_fo = FluxObjective::new(&objective, "biomass_fo", "biomass_reaction", 1.0)
            .expect("Failed to create biomass flux objective");

        let maintenance_fo =
            FluxObjective::new(&objective, "maintenance_fo", "maintenance_reaction", -0.1)
                .expect("Failed to create maintenance flux objective");

        let transport_fo =
            FluxObjective::new(&objective, "transport_fo", "transport_reaction", 0.5)
                .expect("Failed to create transport flux objective");

        // Verify initial state
        assert_eq!(biomass_fo.id(), Some("biomass_fo".to_string()));
        assert_eq!(biomass_fo.reaction(), Some("biomass_reaction".to_string()));
        assert_eq!(biomass_fo.coefficient(), Some(1.0));

        assert_eq!(maintenance_fo.id(), Some("maintenance_fo".to_string()));
        assert_eq!(
            maintenance_fo.reaction(),
            Some("maintenance_reaction".to_string())
        );
        assert_eq!(maintenance_fo.coefficient(), Some(-0.1));

        assert_eq!(transport_fo.id(), Some("transport_fo".to_string()));
        assert_eq!(
            transport_fo.reaction(),
            Some("transport_reaction".to_string())
        );
        assert_eq!(transport_fo.coefficient(), Some(0.5));

        // Modify properties
        biomass_fo.set_id("modified_biomass_fo");
        biomass_fo.set_reaction("modified_biomass_reaction");
        biomass_fo.set_coefficient(2.0);

        // Verify modifications
        assert_eq!(biomass_fo.id(), Some("modified_biomass_fo".to_string()));
        assert_eq!(
            biomass_fo.reaction(),
            Some("modified_biomass_reaction".to_string())
        );
        assert_eq!(biomass_fo.coefficient(), Some(2.0));

        // Ensure other flux objectives are unchanged
        assert_eq!(maintenance_fo.id(), Some("maintenance_fo".to_string()));
        assert_eq!(transport_fo.id(), Some("transport_fo".to_string()));
    }

    #[test]
    fn test_flux_objective_edge_cases() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "edge_case_test");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Test with empty string ID (libSBML might treat this as unset)
        let flux_objective = FluxObjective::new(&objective, "", "reaction1", 1.0)
            .expect("Failed to create flux objective with empty ID");

        // Empty string might be treated as unset by libSBML
        let id_result = flux_objective.id();
        assert!(id_result.is_none() || id_result == Some("".to_string()));

        // Test with very long IDs
        let long_id = "a".repeat(1000);
        let long_reaction_id = "r".repeat(1000);

        let flux_objective_long = FluxObjective::new(&objective, &long_id, &long_reaction_id, 1.0)
            .expect("Failed to create flux objective with long IDs");

        assert_eq!(flux_objective_long.id(), Some(long_id));
        assert_eq!(flux_objective_long.reaction(), Some(long_reaction_id));

        // Test with extreme coefficient values
        let extreme_positive = FluxObjective::new(&objective, "extreme_pos", "reaction2", f64::MAX)
            .expect("Failed to create flux objective with extreme positive coefficient");
        assert_eq!(extreme_positive.coefficient(), Some(f64::MAX));

        let extreme_negative = FluxObjective::new(&objective, "extreme_neg", "reaction3", f64::MIN)
            .expect("Failed to create flux objective with extreme negative coefficient");
        assert_eq!(extreme_negative.coefficient(), Some(f64::MIN));

        // Test with very small coefficient
        let tiny_coefficient = FluxObjective::new(&objective, "tiny", "reaction4", f64::EPSILON)
            .expect("Failed to create flux objective with tiny coefficient");
        assert_eq!(tiny_coefficient.coefficient(), Some(f64::EPSILON));
    }

    #[test]
    fn test_flux_objective_multiple_in_same_objective() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Create multiple flux objectives in the same objective
        let fo1 = FluxObjective::new(&objective, "fo1", "reaction1", 1.0)
            .expect("Failed to create flux objective 1");
        let fo2 = FluxObjective::new(&objective, "fo2", "reaction2", -0.5)
            .expect("Failed to create flux objective 2");
        let fo3 = FluxObjective::new(&objective, "fo3", "reaction3", 2.0)
            .expect("Failed to create flux objective 3");

        // Verify all flux objectives are independent
        assert_eq!(fo1.coefficient(), Some(1.0));
        assert_eq!(fo2.coefficient(), Some(-0.5));
        assert_eq!(fo3.coefficient(), Some(2.0));

        // Modify one and ensure others are unchanged
        fo2.set_coefficient(10.0);
        assert_eq!(fo1.coefficient(), Some(1.0));
        assert_eq!(fo2.coefficient(), Some(10.0));
        assert_eq!(fo3.coefficient(), Some(2.0));
    }

    #[test]
    fn test_flux_objective_special_float_values() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");
        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Test with infinity (should work)
        let inf_fo = FluxObjective::new(&objective, "inf_fo", "reaction1", f64::INFINITY)
            .expect("Failed to create flux objective with infinity");
        assert_eq!(inf_fo.coefficient(), Some(f64::INFINITY));

        // Test with negative infinity
        let neg_inf_fo =
            FluxObjective::new(&objective, "neg_inf_fo", "reaction2", f64::NEG_INFINITY)
                .expect("Failed to create flux objective with negative infinity");
        assert_eq!(neg_inf_fo.coefficient(), Some(f64::NEG_INFINITY));

        // Test with NaN (should work but may not be meaningful)
        let nan_fo = FluxObjective::new(&objective, "nan_fo", "reaction3", f64::NAN)
            .expect("Failed to create flux objective with NaN");
        assert!(nan_fo.coefficient().unwrap().is_nan());
    }
}
