//! This module provides a safe Rust interface to the libSBML Compartment class.
//!
//! The Compartment class represents a compartment in an SBML model.
//! It can represent a physical space, a cell, or any other entity that can contain species.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Compartment class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin};

use cxx::let_cxx_string;

use crate::{model::Model, sbmlcxx};

/// A safe wrapper around the libSBML Compartment class.
///
/// This struct maintains a reference to the underlying C++ Compartment object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct Compartment<'a> {
    compartment: RefCell<Pin<&'a mut sbmlcxx::Compartment>>,
}

impl<'a> Compartment<'a> {
    /// Creates a new Compartment instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this compartment
    /// * `id` - The identifier for this compartment
    ///
    /// # Returns
    /// A new Species instance
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let compartment_ptr = model.inner().borrow_mut().as_mut().createCompartment();
        let compartment_ref: &mut sbmlcxx::Compartment = unsafe { &mut *compartment_ptr };

        let mut pinned_compartment = unsafe { Pin::new_unchecked(compartment_ref) };

        // Set the id of the compartment
        let_cxx_string!(id = id);
        pinned_compartment.as_mut().setId(&id);

        Self {
            compartment: RefCell::new(pinned_compartment),
        }
    }

    /// Gets the species' identifier.
    ///
    /// # Returns
    /// The species' ID as a String
    pub fn id(&self) -> String {
        self.compartment
            .borrow()
            .getId()
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Sets the compartment's identifier.
    ///
    /// # Arguments
    /// * `id` - The new identifier to set
    pub fn set_id(&self, id: &str) {
        let_cxx_string!(id = id);
        self.compartment.borrow_mut().as_mut().setId(&id);
    }

    /// Gets the compartment's name.
    ///
    /// # Returns
    /// The compartment's name as a String
    pub fn name(&self) -> String {
        self.compartment
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
        self.compartment.borrow_mut().as_mut().setName(&name);
    }
}
