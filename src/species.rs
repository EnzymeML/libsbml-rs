//! This module provides a safe Rust interface to the libSBML Species class.
//!
//! The Species class represents a chemical or biological entity in an SBML model.
//! It can represent molecules, ions, proteins, or any other entity that participates
//! in reactions. Each species can have properties like initial amount/concentration,
//! boundary conditions, and compartment location.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Species class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin};

use cxx::let_cxx_string;

use crate::{model::Model, sbmlcxx};

/// A safe wrapper around the libSBML Species class.
///
/// This struct maintains a reference to the underlying C++ Species object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct Species<'a> {
    species: RefCell<Pin<&'a mut sbmlcxx::Species>>,
}

impl<'a> Species<'a> {
    /// Creates a new Species instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this species
    /// * `id` - The identifier for this species
    ///
    /// # Returns
    /// A new Species instance
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let species_ptr = model.inner().borrow_mut().as_mut().createSpecies();
        let species_ref: &mut sbmlcxx::Species = unsafe { &mut *species_ptr };

        let mut pinned_species = unsafe { Pin::new_unchecked(species_ref) };

        // Set the id of the species
        let_cxx_string!(id = id);
        pinned_species.as_mut().setId(&id);

        Self {
            species: RefCell::new(pinned_species),
        }
    }

    /// Gets the species' identifier.
    ///
    /// # Returns
    /// The species' ID as a String
    pub fn id(&self) -> String {
        self.species.borrow().getId().to_str().unwrap().to_string()
    }

    /// Sets the species' identifier.
    ///
    /// # Arguments
    /// * `id` - The new identifier to set
    pub fn set_id(&self, id: &str) {
        let_cxx_string!(id = id);
        self.species.borrow_mut().as_mut().setId(&id);
    }

    /// Gets the species' name.
    ///
    /// # Returns
    /// The species' name as a String
    pub fn name(&self) -> String {
        self.species
            .borrow()
            .getName()
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Sets the species' name.
    ///
    /// # Arguments
    /// * `name` - The new name to set
    pub fn set_name(&self, name: &str) {
        let_cxx_string!(name = name);
        self.species.borrow_mut().as_mut().setName(&name);
    }

    /// Gets the compartment where this species is located.
    ///
    /// # Returns
    /// The compartment identifier as a String
    pub fn compartment(&self) -> String {
        self.species
            .borrow()
            .getCompartment()
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Sets the compartment where this species is located.
    ///
    /// # Arguments
    /// * `compartment` - The identifier of the compartment
    pub fn set_compartment(&self, compartment: &str) {
        let_cxx_string!(compartment = compartment);
        self.species
            .borrow_mut()
            .as_mut()
            .setCompartment(&compartment);
    }

    /// Gets the initial amount of this species.
    ///
    /// # Returns
    /// The initial amount as a f64
    pub fn initial_amount(&self) -> f64 {
        self.species.borrow().getInitialAmount()
    }

    /// Sets the initial amount of this species.
    ///
    /// # Arguments
    /// * `initial_amount` - The initial amount to set
    pub fn set_initial_amount(&self, initial_amount: f64) {
        self.species
            .borrow_mut()
            .as_mut()
            .setInitialAmount(initial_amount);
    }

    /// Gets the initial concentration of this species.
    ///
    /// # Returns
    /// The initial concentration as a f64
    pub fn initial_concentration(&self) -> f64 {
        self.species.borrow().getInitialConcentration()
    }

    /// Sets the initial concentration of this species.
    ///
    /// # Arguments
    /// * `initial_concentration` - The initial concentration to set
    pub fn set_initial_concentration(&self, initial_concentration: f64) {
        self.species
            .borrow_mut()
            .as_mut()
            .setInitialConcentration(initial_concentration);
    }

    /// Gets whether this species has a boundary condition.
    ///
    /// A boundary species is one whose value is not determined by any rule or reaction
    /// in the model but is set by some external mechanism.
    ///
    /// # Returns
    /// true if this species has a boundary condition, false otherwise
    pub fn boundary_condition(&self) -> bool {
        self.species.borrow().getBoundaryCondition()
    }

    /// Sets whether this species has a boundary condition.
    ///
    /// # Arguments
    /// * `boundary_condition` - Whether this species should have a boundary condition
    pub fn set_boundary_condition(&self, boundary_condition: bool) {
        self.species
            .borrow_mut()
            .as_mut()
            .setBoundaryCondition(boundary_condition);
    }

    /// Gets whether this species is constant.
    ///
    /// A constant species is one whose value cannot be changed by any reaction or rule.
    ///
    /// # Returns
    /// true if this species is constant, false otherwise
    pub fn constant(&self) -> bool {
        self.species.borrow().getConstant()
    }

    /// Sets whether this species is constant.
    ///
    /// # Arguments
    /// * `constant` - Whether this species should be constant
    pub fn set_constant(&self, constant: bool) {
        self.species.borrow_mut().as_mut().setConstant(constant);
    }

    /// Gets whether this species has only substance units.
    ///
    /// If true, the units of the species' amount is interpreted as substance units only,
    /// rather than substance/size units.
    ///
    /// # Returns
    /// true if this species has only substance units, false otherwise
    pub fn has_only_substance_units(&self) -> bool {
        self.species.borrow().getHasOnlySubstanceUnits()
    }

    /// Sets whether this species has only substance units.
    ///
    /// # Arguments
    /// * `has_only_substance_units` - Whether this species should have only substance units
    pub fn set_has_only_substance_units(&self, has_only_substance_units: bool) {
        self.species
            .borrow_mut()
            .as_mut()
            .setHasOnlySubstanceUnits(has_only_substance_units);
    }

    pub fn annotation(&self) {}
}
