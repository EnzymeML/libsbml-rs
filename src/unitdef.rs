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

use crate::{
    model::Model,
    sbmlcxx::{self},
};

/// A safe wrapper around the libSBML Species class.
///
/// This struct maintains a reference to the underlying C++ Species object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct UnitDefinition<'a> {
    unit_definition: RefCell<Pin<&'a mut sbmlcxx::UnitDefinition>>,
}

impl<'a> UnitDefinition<'a> {
    /// Creates a new Unit instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this unit
    /// * `id` - The identifier for this unit
    ///
    /// # Returns
    /// A new Species instance
    pub fn new(model: &Model<'a>, id: &str, name: &str) -> Self {
        let unit_definition_ptr = model.inner().borrow_mut().as_mut().createUnitDefinition();
        let unit_definition_ref: &mut sbmlcxx::UnitDefinition =
            unsafe { &mut *unit_definition_ptr };

        let mut pinned_unit_definition = unsafe { Pin::new_unchecked(unit_definition_ref) };

        let_cxx_string!(id = id);
        pinned_unit_definition.as_mut().setId(&id);

        let_cxx_string!(name = name);
        pinned_unit_definition.as_mut().setName(&name);

        Self {
            unit_definition: RefCell::new(pinned_unit_definition),
        }
    }

    pub(crate) fn inner(&self) -> &RefCell<Pin<&'a mut sbmlcxx::UnitDefinition>> {
        &self.unit_definition
    }
}
