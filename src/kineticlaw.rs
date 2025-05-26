//! This module provides a safe Rust interface to the libSBML KineticLaw class.
//!
//! The KineticLaw class represents a mathematical expression that defines the rate
//! at which a reaction occurs in an SBML model. It encapsulates the quantitative
//! aspects of reaction kinetics, such as mass action, Michaelis-Menten, or other
//! custom rate laws.
//!
//! KineticLaws can include:
//! - Mathematical formulas (e.g., "k * S1 * S2")
//! - Local parameters specific to the reaction
//! - References to global parameters and species
//! - Units of measurement for the rate
//!
//! This wrapper provides safe access to the underlying C++ libSBML KineticLaw class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc};

use cxx::let_cxx_string;

use crate::{
    clone, inner, pin_ptr,
    prelude::{LocalParameter, LocalParameterBuilder, Reaction},
    required_property, sbase, sbmlcxx, sbo_term,
    traits::fromptr::FromPtr,
    upcast_annotation,
};

/// A safe wrapper around the libSBML KineticLaw class.
///
/// This struct maintains a reference to the underlying C++ KineticLaw object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
///
/// KineticLaw objects define the mathematical rules that govern reaction rates in
/// systems biology models. They can range from simple mass action kinetics to complex
/// enzymatic rate laws.
pub struct KineticLaw<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::KineticLaw>>,
    local_parameters: RefCell<Vec<Rc<LocalParameter<'a>>>>,
}

// Set the inner trait for the KineticLaw struct
inner!(sbmlcxx::KineticLaw, KineticLaw<'a>);

// Set the sbase trait for the KineticLaw struct
sbase!(KineticLaw<'a>, sbmlcxx::KineticLaw);

// Set the annotation trait for the KineticLaw struct
upcast_annotation!(KineticLaw<'a>, sbmlcxx::KineticLaw, sbmlcxx::SBase);

// Implement the Clone trait for the KineticLaw struct
clone!(KineticLaw<'a>, sbmlcxx::KineticLaw, local_parameters);

impl<'a> KineticLaw<'a> {
    /// Creates a new KineticLaw instance for the given Reaction.
    ///
    /// This method creates a new kinetic law with the specified mathematical formula
    /// and associates it with the provided reaction. The formula defines how the
    /// reaction rate is calculated based on species concentrations, parameters, and
    /// other model elements.
    ///
    /// # Arguments
    /// * `reaction` - The parent Reaction that will contain this kinetic law
    /// * `formula` - The mathematical formula for this kinetic law (e.g., "k1 * S1")
    ///
    /// # Returns
    /// A new KineticLaw instance initialized with the given formula and added to the reaction
    pub fn new(reaction: &Reaction<'a>, formula: &str) -> Self {
        let kinetic_law_ptr = reaction.inner().borrow_mut().as_mut().createKineticLaw();
        let mut kinetic_law = pin_ptr!(kinetic_law_ptr, sbmlcxx::KineticLaw);

        let_cxx_string!(formula = formula);
        kinetic_law.as_mut().setFormula(&formula);

        Self {
            inner: RefCell::new(kinetic_law),
            local_parameters: RefCell::new(vec![]),
        }
    }

    // Getter and setter for formula
    required_property!(KineticLaw<'a>, formula, String, getFormula, setFormula);

    /// Gets the local parameters of the kinetic law.
    ///
    /// This method retrieves all local parameters associated with the kinetic law.
    /// Local parameters are specific to the kinetic law and not shared with other
    /// kinetic laws or reactions.
    ///
    /// # Returns
    /// A vector of LocalParameter instances representing the local parameters of the kinetic law.
    pub fn local_parameters(&self) -> Vec<Rc<LocalParameter<'a>>> {
        self.local_parameters.borrow().to_vec()
    }

    /// Adds a local parameter to the kinetic law.
    ///
    /// This method adds a new local parameter to the kinetic law. Local parameters
    /// are specific to the kinetic law and not shared with other kinetic laws or reactions.
    ///
    /// # Arguments
    /// * `id` - The identifier of the local parameter
    /// * `value` - The value of the local parameter
    ///
    /// # Returns
    /// A new LocalParameter instance representing the added local parameter
    pub fn add_local_parameter(&self, id: &str, value: Option<f64>) -> Rc<LocalParameter<'a>> {
        let local_parameter = Rc::new(LocalParameter::new(self, id));

        if let Some(value) = value {
            local_parameter.set_value(value);
        }

        self.local_parameters
            .borrow_mut()
            .push(Rc::clone(&local_parameter));

        local_parameter
    }

    /// Creates a LocalParameterBuilder for constructing a LocalParameter with a fluent API.
    ///
    /// This method provides a builder pattern interface for creating and configuring
    /// a new LocalParameter within this kinetic law. The builder allows chaining method calls
    /// to set various properties of the LocalParameter before building it.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new local parameter
    ///
    /// # Returns
    /// A LocalParameterBuilder instance that can be used to configure and create the LocalParameter
    pub fn build_local_parameter(&self, id: &str) -> LocalParameterBuilder<'a> {
        LocalParameterBuilder::new(self, id)
    }

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::KineticLaw, sbmlcxx::SBase);
}

impl FromPtr<sbmlcxx::KineticLaw> for KineticLaw<'_> {
    /// Creates a KineticLaw instance from a raw pointer to a libSBML KineticLaw.
    ///
    /// This implementation allows converting from a raw C++ pointer to a safe Rust wrapper.
    /// It's primarily used internally by the library.
    ///
    /// # Arguments
    /// * `ptr` - Raw pointer to a libSBML KineticLaw object
    ///
    /// # Returns
    /// A new KineticLaw instance wrapping the provided pointer
    fn from_ptr(ptr: *mut sbmlcxx::KineticLaw) -> Self {
        let mut kinetic_law = pin_ptr!(ptr, sbmlcxx::KineticLaw);
        let n_local_parameters = kinetic_law.as_mut().getNumLocalParameters().0;
        let local_parameters: Vec<_> = (0..n_local_parameters)
            .map(|i| {
                let local_parameter = kinetic_law.as_mut().getLocalParameter1(i.into());
                Rc::new(LocalParameter::from_ptr(local_parameter))
            })
            .collect();

        Self {
            inner: RefCell::new(kinetic_law),
            local_parameters: RefCell::new(local_parameters),
        }
    }
}

impl std::fmt::Debug for KineticLaw<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("KineticLaw");
        ds.field("formula", &self.formula());
        ds.field("local_parameters", &self.local_parameters());
        ds.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{model::Model, reaction::Reaction, SBMLDocument};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_kinetic_law_new() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let reaction = Reaction::new(&model, "r1");
        let kinetic_law = KineticLaw::new(&reaction, "k1 * S1");
        assert_eq!(kinetic_law.formula(), "k1 * S1");
        assert_eq!(kinetic_law.local_parameters().len(), 0);
    }

    #[test]
    fn test_kinetic_law_local_parameters() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let reaction = Reaction::new(&model, "r1");
        let kinetic_law = KineticLaw::new(&reaction, "k1 * S1");
        kinetic_law.add_local_parameter("k1", Some(1.0));

        assert_eq!(kinetic_law.local_parameters().len(), 1);
        assert_eq!(kinetic_law.local_parameters()[0].id(), "k1");
        assert_eq!(kinetic_law.local_parameters()[0].value(), Some(1.0));
    }

    #[test]
    fn test_kinetic_law_build_local_parameter() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let reaction = Reaction::new(&model, "r1");
        let kinetic_law = KineticLaw::new(&reaction, "k1 * S1");
        let local_parameter = kinetic_law.build_local_parameter("k1").value(1.0).build();
        assert_eq!(local_parameter.id(), "k1");
        assert_eq!(local_parameter.value(), Some(1.0));
    }

    #[test]
    fn test_set_formula() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let reaction = Reaction::new(&model, "r1");
        let kinetic_law = KineticLaw::new(&reaction, "k1 * S1");
        kinetic_law.set_formula("k2 * S2");
        assert_eq!(kinetic_law.formula(), "k2 * S2");
    }

    #[test]
    fn test_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let reaction = Reaction::new(&model, "r1");
        let kinetic_law = KineticLaw::new(&reaction, "k1 * S1");
        kinetic_law
            .set_annotation("<test>test</test>")
            .expect("Failed to set annotation");
        assert_eq!(
            kinetic_law
                .get_annotation()
                .replace("\n", "")
                .replace(' ', ""),
            "<annotation><test>test</test></annotation>"
        );
    }

    #[test]
    fn test_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let reaction = Reaction::new(&model, "r1");
        let kinetic_law = KineticLaw::new(&reaction, "k1 * S1");

        kinetic_law
            .set_annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .expect("Failed to set annotation");

        let annotation = kinetic_law
            .get_annotation_serde::<TestAnnotation>()
            .expect("Failed to deserialize annotation");

        assert_eq!(annotation.test, "test");
    }

    #[test]
    fn test_sbo_term() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let reaction = Reaction::new(&model, "r1");
        let kinetic_law = KineticLaw::new(&reaction, "k1 * S1");

        kinetic_law.set_sbo_term("SBO:0000001");
        assert_eq!(kinetic_law.sbo_term_id(), "SBO:0000001");
        assert!(kinetic_law.sbo_term_url().contains("SBO:0000001"));
    }

    #[test]
    fn test_clone() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let reaction = Reaction::new(&model, "r1");
        let kinetic_law = KineticLaw::new(&reaction, "k1 * S1");

        let cloned_kinetic_law = kinetic_law.clone();
        assert_eq!(kinetic_law.formula(), cloned_kinetic_law.formula());

        // Modify the clone and verify it doesn't affect the original
        cloned_kinetic_law.set_formula("k2 * S2");
        assert_eq!(kinetic_law.formula(), "k1 * S1");
        assert_eq!(cloned_kinetic_law.formula(), "k2 * S2");
    }
}
