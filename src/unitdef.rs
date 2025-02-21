//! This module provides a safe Rust interface to the libSBML UnitDefinition class.
//!
//! The UnitDefinition class represents a unit definition in an SBML model.
//! It can represent a unit, a unit system, or any other entity that can be used to define the units of a model.
//! Each unit definition can have properties like name, kind, and exponent.
//!
//! This wrapper provides safe access to the underlying C++ libSBML UnitDefinition class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin};

use cxx::let_cxx_string;

use crate::{
    model::Model,
    sbmlcxx::{self},
};

/// A safe wrapper around the libSBML UnitDefinition class.
///
/// This struct maintains a reference to the underlying C++ UnitDefinition object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct UnitDefinition<'a> {
    unit_definition: RefCell<Pin<&'a mut sbmlcxx::UnitDefinition>>,
}

impl<'a> UnitDefinition<'a> {
    /// Creates a new UnitDefinition instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this unit definition
    /// * `id` - The identifier for this unit definition
    ///
    /// # Returns
    /// A new UnitDefinition instance
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
