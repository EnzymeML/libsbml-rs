//! This module provides a safe Rust interface to the libSBML UnitDefinition class.
//!
//! The UnitDefinition class represents a unit definition in an SBML model.
//! It can represent a unit, a unit system, or any other entity that can be used to define the units of a model.
//! Each unit definition can have properties like name, kind, and exponent.
//!
//! This wrapper provides safe access to the underlying C++ libSBML UnitDefinition class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, error::Error, pin::Pin, rc::Rc};

use cxx::let_cxx_string;
use quick_xml::{de::from_str, se::to_string, DeError, SeError};
use serde::{Deserialize, Serialize};

use crate::{
    model::Model,
    pin_ptr,
    sbmlcxx::{self},
    unit::Unit,
    Annotation,
};

/// A safe wrapper around the libSBML UnitDefinition class.
///
/// This struct maintains a reference to the underlying C++ UnitDefinition object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct UnitDefinition<'a> {
    unit_definition: RefCell<Pin<&'a mut sbmlcxx::UnitDefinition>>,
    units: RefCell<Vec<Rc<Unit<'a>>>>,
}

impl<'a> UnitDefinition<'a> {
    /// Creates a new UnitDefinition instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this unit definition
    /// * `id` - The identifier for this unit definition
    /// * `name` - The name of this unit definition
    ///
    /// # Returns
    /// A new UnitDefinition instance
    pub fn new(model: &Model<'a>, id: &str, name: &str) -> Self {
        let unit_definition_ptr = model.inner().borrow_mut().as_mut().createUnitDefinition();
        let mut unit_definition = pin_ptr!(unit_definition_ptr, sbmlcxx::UnitDefinition);

        let_cxx_string!(id = id);
        unit_definition.as_mut().setId(&id);

        let_cxx_string!(name = name);
        unit_definition.as_mut().setName(&name);

        Self {
            unit_definition: RefCell::new(unit_definition),
            units: RefCell::new(Vec::new()),
        }
    }

    /// Returns a reference to the inner RefCell containing the UnitDefinition pointer.
    ///
    /// This is primarily used internally by other parts of the library.
    pub(crate) fn inner(&self) -> &RefCell<Pin<&'a mut sbmlcxx::UnitDefinition>> {
        &self.unit_definition
    }

    /// Adds a unit to the unit definition.
    ///
    /// # Arguments
    /// * `unit` - The unit to add to the unit definition
    pub fn add_unit(&self, unit: Rc<Unit<'a>>) {
        self.units.borrow_mut().push(Rc::clone(&unit));
    }

    /// Returns a vector of all units in the unit definition.
    ///
    /// # Returns
    /// A vector of all units in the unit definition
    pub fn units(&self) -> Vec<Rc<Unit<'a>>> {
        self.units.borrow().to_vec()
    }
}

impl<'a> Annotation for UnitDefinition<'a> {
    /// Gets the annotation for the species.
    ///
    /// # Returns
    /// The species' annotation as a String
    fn get_annotation(&self) -> String {
        let annotation = unsafe {
            sbmlcxx::utils::getUnitDefinitionAnnotationString(
                self.unit_definition
                    .borrow_mut()
                    .as_mut()
                    .get_unchecked_mut(),
            )
        };
        annotation.to_str().unwrap().to_string()
    }

    /// Sets the annotation for the species.
    ///
    /// # Arguments
    /// * `annotation` - A string slice that holds the annotation to set.
    fn set_annotation(&self, annotation: &str) -> Result<(), Box<dyn Error>> {
        let_cxx_string!(annotation = annotation);
        unsafe {
            sbmlcxx::utils::setUnitDefinitionAnnotation(
                self.unit_definition
                    .borrow_mut()
                    .as_mut()
                    .get_unchecked_mut(),
                &annotation,
            );
        }
        Ok(())
    }

    /// Sets the annotation for the species using a serializable type.
    ///
    /// # Arguments
    /// * `annotation` - A serializable type that holds the annotation to set.
    fn set_annotation_serde<T: Serialize>(&self, annotation: &T) -> Result<(), SeError> {
        let annotation = to_string(annotation)?;
        self.set_annotation(&annotation)
            .map_err(|e| SeError::Custom(e.to_string()))?;
        Ok(())
    }

    /// Gets the annotation for the species using a deserializable type.
    ///
    /// # Returns
    /// The species' annotation as a deserializable type
    fn get_annotation_serde<T: for<'de> Deserialize<'de>>(&self) -> Result<T, DeError> {
        let annotation = self.get_annotation();
        let annotation = from_str(&annotation).unwrap();
        Ok(annotation)
    }
}
