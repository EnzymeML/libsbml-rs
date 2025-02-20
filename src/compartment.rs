//! This module provides a safe Rust interface to the libSBML Compartment class.
//!
//! The Compartment class represents a compartment in an SBML model.
//! It can represent a physical space, a cell, or any other entity that can contain species.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Compartment class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc};

use cxx::let_cxx_string;
use quick_xml::{de::from_str, se::to_string, DeError};
use serde::{Deserialize, Serialize};

use crate::{
    model::Model,
    sbmlcxx::{self, utils},
    wrapper::Wrapper,
    Annotation,
};

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
    /// A new Compartment instance
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

    /// Gets the compartment's identifier.
    ///
    /// # Returns
    /// The compartment's ID as a String
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

    /// Sets the compartment's name.
    ///
    /// # Arguments
    /// * `name` - The new name to set
    pub fn set_name(&self, name: &str) {
        let_cxx_string!(name = name);
        self.compartment.borrow_mut().as_mut().setName(&name);
    }
}

impl<'a> Annotation for Compartment<'a> {
    /// Gets the annotation for the compartment.
    ///
    /// # Returns
    /// The compartment's annotation as a String
    fn get_annotation(&self) -> String {
        let annotation = unsafe {
            utils::getCompartmentAnnotationString(
                self.compartment.borrow_mut().as_mut().get_unchecked_mut(),
            )
        };
        annotation.to_str().unwrap().to_string()
    }

    /// Sets the annotation for the compartment.
    ///
    /// This function allows you to set a string annotation for the compartment,
    /// which can be used to provide additional information or metadata.
    ///
    /// # Arguments
    /// * `annotation` - A string slice that holds the annotation to set.
    fn set_annotation(&self, annotation: &str) {
        let_cxx_string!(annotation = annotation);
        unsafe {
            utils::setCompartmentAnnotation(
                self.compartment.borrow_mut().as_mut().get_unchecked_mut(),
                &annotation,
            );
        }
    }

    /// Sets the annotation for the compartment using a serializable type.
    ///
    /// This function serializes the provided annotation into a string format
    /// and sets it as the compartment's annotation. It is useful for complex
    /// data structures that can be serialized.
    ///
    /// # Arguments
    /// * `annotation` - A reference to a serializable type that will be converted to a string.
    fn set_annotation_serde<T: Serialize>(&self, annotation: &T) {
        let annotation = to_string(annotation).unwrap();
        self.set_annotation(&annotation);
    }

    /// Gets the annotation for the compartment as a serializable type.
    ///
    /// This function deserializes the compartment's annotation from a string format
    /// into the specified type. It is useful for complex data structures that
    /// can be deserialized.
    ///
    /// # Returns
    /// The deserialized annotation as the specified type
    fn get_annotation_serde<T: for<'de> Deserialize<'de>>(&self) -> Result<T, DeError> {
        let annotation = self.get_annotation();
        let parsed: Wrapper<T> = from_str(&annotation)?;
        Ok(parsed.annotation)
    }
}

/// A builder for constructing Compartment instances with a fluent API.
///
/// This struct provides a builder pattern interface for creating and configuring
/// Compartment objects. It allows chaining method calls to set various properties
/// before finally constructing the Compartment.
///
/// # Example
/// ```no_run
/// use sbml::prelude::*;
///
/// let doc = SBMLDocument::new(3, 2);
/// let model = Model::new(&doc, "test");
/// let compartment = model.build_compartment("cytosol")
///     .name("Cytosol")
///     .build();
/// ```
pub struct CompartmentBuilder<'a> {
    compartment: Rc<Compartment<'a>>,
}

impl<'a> CompartmentBuilder<'a> {
    /// Creates a new CompartmentBuilder.
    ///
    /// # Arguments
    /// * `model` - The model that will contain the compartment
    /// * `id` - The identifier for the new compartment
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let compartment = Rc::new(Compartment::new(model, id));
        Self { compartment }
    }

    /// Sets the name of the compartment.
    ///
    /// # Arguments
    /// * `name` - The name to set
    pub fn name(self, name: &str) -> Self {
        self.compartment.set_name(name);
        self
    }

    /// Sets the annotation for this compartment.
    ///
    /// # Arguments
    /// * `annotation` - The annotation string to set
    pub fn annotation(self, annotation: &str) -> Self {
        self.compartment.set_annotation(annotation);
        self
    }

    /// Sets a serializable annotation for this compartment.
    ///
    /// # Arguments
    /// * `annotation` - The annotation to serialize and set
    pub fn annotation_serde<T: Serialize>(self, annotation: &T) -> Self {
        let annotation = to_string(annotation).expect("Failed to serialize annotation");
        self.compartment.set_annotation(&annotation);
        self
    }

    /// Builds and returns the configured Compartment instance.
    pub fn build(self) -> Rc<Compartment<'a>> {
        self.compartment
    }
}
