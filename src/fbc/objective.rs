//! This module provides a safe Rust interface to the libSBML Objective class.
//!
//! The Objective class represents an optimization objective in an SBML FBC (Flux Balance Constraints) model.
//! Objectives define what should be optimized in flux balance analysis, typically maximizing or minimizing
//! a linear combination of reaction fluxes. Each objective contains one or more flux objectives that
//! specify the reactions and their coefficients in the objective function.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Objective class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc};

use cxx::let_cxx_string;

use crate::{
    clone, errors::LibSBMLError, inner, model::Model, pin_ptr, plugin::get_plugin, prelude::IntoId,
    required_property, sbmlcxx, traits::fromptr::FromPtr, upcast_annotation,
};

use super::{fluxobjective::FluxObjective, objectivetype::ObjectiveType};

/// A safe wrapper around the libSBML Objective class.
///
/// Objective represents an optimization objective in an SBML FBC model. It consists of:
/// - An identifier (required)
/// - An objective type (maximize or minimize)
/// - A collection of flux objectives that define the linear combination of reaction fluxes
///
/// This struct maintains a reference to the underlying C++ Objective object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
/// It also maintains a collection of FluxObjective instances associated with this objective.
pub struct Objective<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::Objective>>,
    list_of_flux_objective: RefCell<Vec<Rc<FluxObjective<'a>>>>,
}

// Set the inner trait for the Compartment struct
inner!(sbmlcxx::Objective, Objective<'a>);

// Set the annotation trait for the Compartment struct
upcast_annotation!(Objective<'a>, sbmlcxx::Objective, sbmlcxx::SBase);

// Implement the Clone trait for the Compartment struct
clone!(Objective<'a>, sbmlcxx::Objective, list_of_flux_objective);

impl<'a> Objective<'a> {
    /// Creates a new Objective instance within the given Model.
    ///
    /// This method creates an optimization objective that can be used in flux balance analysis.
    /// The objective defines whether to maximize or minimize a linear combination of reaction fluxes.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this objective
    /// * `id` - The identifier for this objective (must be unique within the model)
    /// * `obj_type` - The type of optimization (maximize or minimize)
    ///
    /// # Returns
    /// A new Objective instance initialized with the given parameters and added to the model
    ///
    /// # Errors
    /// Returns `LibSBMLError` if:
    /// - The FBC plugin is not available or enabled in the model
    /// - The objective creation fails in the underlying libSBML library
    pub fn new(
        model: &Model<'a>,
        id: &str,
        obj_type: impl Into<ObjectiveType>,
    ) -> Result<Self, LibSBMLError> {
        let mut fbc_plugin =
            get_plugin::<sbmlcxx::FbcModelPlugin, Model<'a>, sbmlcxx::Model>(model, "fbc")?;

        // Create the objective
        let objective_ptr = fbc_plugin.as_mut().createObjective();
        let mut objective = pin_ptr!(objective_ptr, sbmlcxx::Objective);

        // Set the id
        let_cxx_string!(id = id);
        objective.as_mut().setId(&id);

        // Set the type
        let obj_type = obj_type.into();
        objective.as_mut().setType(obj_type.into());

        Ok(Self {
            inner: RefCell::new(objective),
            list_of_flux_objective: RefCell::new(vec![]),
        })
    }

    // Setter and getter for id
    required_property!(Objective<'a>, id, String, getId, setId);

    // Setter and getter for name
    required_property!(
        Objective<'a>,
        obj_type,
        ObjectiveType,
        getObjectiveType,
        setType
    );

    /// Creates a new FluxObjective instance within this Objective.
    ///
    /// Flux objectives define the individual reaction contributions to the overall objective function.
    /// Each flux objective specifies a reaction and its coefficient in the linear combination.
    ///
    /// # Arguments
    /// * `id` - The identifier for this flux objective
    /// * `reaction_id` - The identifier for the reaction that contributes to the objective
    /// * `coefficient` - The coefficient (weight) of this reaction in the objective function
    ///
    /// # Returns
    /// A new FluxObjective instance wrapped in an Rc, or an error if creation fails
    pub fn create_flux_objective(
        &self,
        id: &str,
        reaction_id: impl IntoId,
        coefficient: f64,
    ) -> Result<Rc<FluxObjective<'a>>, LibSBMLError> {
        let flux_objective = Rc::new(FluxObjective::new(self, id, reaction_id, coefficient)?);

        self.list_of_flux_objective
            .borrow_mut()
            .push(Rc::clone(&flux_objective));
        Ok(flux_objective)
    }

    /// Returns a list of all FluxObjective instances associated with this Objective.
    ///
    /// # Returns
    /// A vector containing Rc references to all FluxObjectives in this objective
    pub fn flux_objectives(&self) -> Vec<Rc<FluxObjective<'a>>> {
        self.list_of_flux_objective.borrow().clone()
    }

    /// Retrieves a flux objective from this objective by its identifier.
    ///
    /// # Arguments
    /// * `id` - The identifier of the flux objective to retrieve
    ///
    /// # Returns
    /// Some(`Rc<FluxObjective>`) if found, None if not found
    pub fn get_flux_objective(&self, id: &str) -> Option<Rc<FluxObjective<'a>>> {
        self.list_of_flux_objective
            .borrow()
            .iter()
            .find(|flux_objective| (*flux_objective).id() == Some(id.to_string()))
            .map(Rc::clone)
    }
}

impl<'a> FromPtr<sbmlcxx::Objective> for Objective<'a> {
    fn from_ptr(ptr: *mut sbmlcxx::Objective) -> Self {
        let mut objective = pin_ptr!(ptr, sbmlcxx::Objective);
        let n_flux_objectives = objective.as_mut().getNumFluxObjectives().0;
        let list_of_flux_objectives: Vec<_> = (0..n_flux_objectives)
            .map(|i| {
                let flux_objective = objective.as_mut().getFluxObjective(i.into());
                let flux_objective = Rc::new(FluxObjective::from_ptr(flux_objective));
                Rc::clone(&flux_objective)
            })
            .collect();

        Self {
            inner: RefCell::new(objective),
            list_of_flux_objective: RefCell::new(list_of_flux_objectives),
        }
    }
}

impl<'a> std::fmt::Debug for Objective<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Objective");
        ds.field("id", &self.id());
        ds.field("obj_type", &self.obj_type());
        ds.field("flux_objectives", &self.flux_objectives());
        ds.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::Model, sbmldoc::SBMLDocument};

    #[test]
    fn test_objective_new() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let objective = Objective::new(&model, "biomass_objective", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        assert_eq!(objective.id(), "biomass_objective");
        assert_eq!(objective.obj_type(), ObjectiveType::Maximize);
    }

    #[test]
    fn test_objective_new_with_different_types() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let maximize_obj = Objective::new(&model, "max_obj", ObjectiveType::Maximize)
            .expect("Failed to create maximize objective");
        assert_eq!(maximize_obj.obj_type(), ObjectiveType::Maximize);

        let minimize_obj = Objective::new(&model, "min_obj", ObjectiveType::Minimize)
            .expect("Failed to create minimize objective");
        assert_eq!(minimize_obj.obj_type(), ObjectiveType::Minimize);
    }

    #[test]
    fn test_objective_id_operations() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let objective = Objective::new(&model, "original_id", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Test initial ID
        assert_eq!(objective.id(), "original_id");

        // Test setting new ID
        objective.set_id("new_id");
        assert_eq!(objective.id(), "new_id");
    }

    #[test]
    fn test_objective_type_operations() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Test initial type
        assert_eq!(objective.obj_type(), ObjectiveType::Maximize);

        // Test setting new type
        objective.set_obj_type(ObjectiveType::Minimize);
        assert_eq!(objective.obj_type(), ObjectiveType::Minimize);
    }

    #[test]
    fn test_objective_create_flux_objective() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        let flux_objective = objective
            .create_flux_objective("fo1", "reaction1", 1.0)
            .expect("Failed to create flux objective");

        assert_eq!(flux_objective.id(), Some("fo1".to_string()));
        assert_eq!(flux_objective.reaction(), Some("reaction1".to_string()));
        assert_eq!(flux_objective.coefficient(), Some(1.0));
    }

    #[test]
    fn test_objective_flux_objectives_list() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        // Initially empty
        assert_eq!(objective.flux_objectives().len(), 0);

        // Add flux objectives
        objective
            .create_flux_objective("fo1", "reaction1", 1.0)
            .expect("Failed to create flux objective");
        objective
            .create_flux_objective("fo2", "reaction2", -0.5)
            .expect("Failed to create flux objective");

        let flux_objectives = objective.flux_objectives();
        assert_eq!(flux_objectives.len(), 2);
    }

    #[test]
    fn test_objective_get_flux_objective() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        objective
            .create_flux_objective("fo1", "reaction1", 1.0)
            .expect("Failed to create flux objective");

        // Test getting existing flux objective
        let flux_objective = objective
            .get_flux_objective("fo1")
            .expect("Failed to get flux objective");
        assert_eq!(flux_objective.id(), Some("fo1".to_string()));

        // Test getting non-existent flux objective
        assert!(objective.get_flux_objective("nonexistent").is_none());
    }

    #[test]
    fn test_objective_debug() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let objective = Objective::new(&model, "debug_obj", ObjectiveType::Minimize)
            .expect("Failed to create objective");

        objective
            .create_flux_objective("fo1", "reaction1", 2.5)
            .expect("Failed to create flux objective");

        let debug_string = format!("{objective:?}");
        assert!(debug_string.contains("Objective"));
        assert!(debug_string.contains("debug_obj"));
        assert!(debug_string.contains("Minimize"));
    }

    #[test]
    fn test_objective_clone() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        let objective = Objective::new(&model, "obj1", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        objective
            .create_flux_objective("fo1", "reaction1", 1.0)
            .expect("Failed to create flux objective");

        // Test that clone works (this tests the clone! macro)
        let cloned = objective.clone();
        assert_eq!(cloned.id(), objective.id());
        assert_eq!(cloned.obj_type(), objective.obj_type());
        assert_eq!(
            cloned.flux_objectives().len(),
            objective.flux_objectives().len()
        );
    }

    #[test]
    fn test_objective_comprehensive_workflow() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "comprehensive_test");

        // Create an objective for biomass maximization
        let biomass_obj = Objective::new(&model, "biomass", ObjectiveType::Maximize)
            .expect("Failed to create biomass objective");

        // Add multiple flux objectives
        biomass_obj
            .create_flux_objective("biomass_fo", "biomass_reaction", 1.0)
            .expect("Failed to create biomass flux objective");

        biomass_obj
            .create_flux_objective("maintenance_fo", "maintenance_reaction", -0.1)
            .expect("Failed to create maintenance flux objective");

        // Verify the objective structure
        assert_eq!(biomass_obj.id(), "biomass");
        assert_eq!(biomass_obj.obj_type(), ObjectiveType::Maximize);
        assert_eq!(biomass_obj.flux_objectives().len(), 2);

        // Test individual flux objectives
        let biomass_fo = biomass_obj
            .get_flux_objective("biomass_fo")
            .expect("Failed to get biomass flux objective");
        assert_eq!(biomass_fo.coefficient(), Some(1.0));

        let maintenance_fo = biomass_obj
            .get_flux_objective("maintenance_fo")
            .expect("Failed to get maintenance flux objective");
        assert_eq!(maintenance_fo.coefficient(), Some(-0.1));

        // Modify the objective
        biomass_obj.set_id("modified_biomass");
        biomass_obj.set_obj_type(ObjectiveType::Minimize);

        assert_eq!(biomass_obj.id(), "modified_biomass");
        assert_eq!(biomass_obj.obj_type(), ObjectiveType::Minimize);
    }

    #[test]
    fn test_objective_with_string_ids() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        // Test with String
        let obj_id = String::from("string_objective");
        let objective = Objective::new(&model, &obj_id, ObjectiveType::Maximize)
            .expect("Failed to create objective");

        assert_eq!(objective.id(), "string_objective");

        // Test flux objective with String reaction ID
        let reaction_id = String::from("string_reaction");
        let flux_objective = objective
            .create_flux_objective("fo1", reaction_id, 1.5)
            .expect("Failed to create flux objective");

        assert_eq!(
            flux_objective.reaction(),
            Some("string_reaction".to_string())
        );
    }

    #[test]
    fn test_objective_edge_cases() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "edge_case_test");

        // Test with empty string ID (libSBML might treat this as unset)
        let objective = Objective::new(&model, "", ObjectiveType::Maximize)
            .expect("Failed to create objective with empty ID");

        // Empty string might be treated as unset by libSBML
        let id_result = objective.id();
        assert!(id_result.is_empty());

        // Test with very long ID
        let long_id = "a".repeat(1000);
        let long_objective = Objective::new(&model, &long_id, ObjectiveType::Minimize)
            .expect("Failed to create objective with long ID");

        assert_eq!(long_objective.id(), long_id);

        // Test flux objective with zero coefficient
        let flux_objective = objective
            .create_flux_objective("zero_fo", "reaction1", 0.0)
            .expect("Failed to create flux objective with zero coefficient");

        assert_eq!(flux_objective.coefficient(), Some(0.0));

        // Test flux objective with negative coefficient
        let negative_flux_objective = objective
            .create_flux_objective("negative_fo", "reaction2", -5.5)
            .expect("Failed to create flux objective with negative coefficient");

        assert_eq!(negative_flux_objective.coefficient(), Some(-5.5));
    }

    #[test]
    fn test_objective_from_ptr() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test_model");

        // Create an objective through the model to test FromPtr
        let objective = Objective::new(&model, "test_obj", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        objective
            .create_flux_objective("fo1", "reaction1", 1.0)
            .expect("Failed to create flux objective");

        // Test that the objective was created correctly
        assert_eq!(objective.id(), "test_obj");
        assert_eq!(objective.obj_type(), ObjectiveType::Maximize);
        assert_eq!(objective.flux_objectives().len(), 1);
    }
}
