//! This module provides a safe Rust interface to the libSBML Model class.
//!
//! The Model class is a core component of SBML (Systems Biology Markup Language),
//! representing a biological model containing species, reactions, compartments and other
//! elements that describe a biological system.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Model class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc};

use cxx::let_cxx_string;
use quick_xml::{de::from_str, se::to_string, DeError};
use serde::{Deserialize, Serialize};

use crate::{
    annotation::Annotation,
    compartment::{Compartment, CompartmentBuilder},
    sbmlcxx::{self, utils},
    sbmldoc::SBMLDocument,
    species::{Species, SpeciesBuilder},
    wrapper::Wrapper,
};

/// A safe wrapper around the libSBML Model class.
///
/// This struct maintains a reference to the underlying C++ Model object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct Model<'a> {
    model: RefCell<Pin<&'a mut sbmlcxx::Model>>,
    species: RefCell<Vec<Rc<Species<'a>>>>,
    compartments: RefCell<Vec<Rc<Compartment<'a>>>>,
}

impl<'a> Model<'a> {
    /// Creates a new Model instance within the given SBMLDocument.
    ///
    /// # Arguments
    /// * `document` - The parent SBMLDocument that will contain this model
    /// * `id` - The identifier for this model
    ///
    /// # Returns
    /// A new Model instance
    pub fn new(document: &SBMLDocument, id: &str) -> Self {
        let model_ptr = document.inner().borrow_mut().pin_mut().createModel(id);
        let model_ref: &mut sbmlcxx::Model = unsafe { &mut *model_ptr };
        let pinned_model = unsafe { Pin::new_unchecked(model_ref) };

        Self {
            model: RefCell::new(pinned_model),
            species: RefCell::new(Vec::new()),
            compartments: RefCell::new(Vec::new()),
        }
    }

    /// Returns a reference to the inner RefCell containing the Model pointer.
    ///
    /// This is primarily used internally by other parts of the library.
    pub(crate) fn inner(&self) -> &RefCell<Pin<&'a mut sbmlcxx::Model>> {
        &self.model
    }

    /// Gets the model's identifier.
    ///
    /// # Returns
    /// The model's ID as a String
    pub fn id(&self) -> String {
        self.model.borrow().getId().to_str().unwrap().to_string()
    }

    /// Sets the model's identifier.
    ///
    /// # Arguments
    /// * `id` - The new identifier to set
    pub fn set_id(&self, id: &str) {
        let_cxx_string!(id = id);
        self.model.borrow_mut().as_mut().setId(&id);
    }

    /// Gets the model's name.
    ///
    /// # Returns
    /// The model's name as a String
    pub fn name(&self) -> String {
        self.model.borrow().getName().to_str().unwrap().to_string()
    }

    /// Sets the model's name.
    ///
    /// # Arguments
    /// * `name` - The new name to set
    pub fn set_name(&self, name: &str) {
        let_cxx_string!(name = name);
        self.model.borrow_mut().as_mut().setName(&name);
    }

    /// Creates a new Species within this model.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new species
    ///
    /// # Returns
    /// A new Species instance
    pub fn create_species(&self, id: &str) -> Rc<Species<'a>> {
        let species = Rc::new(Species::new(self, id));
        self.species.borrow_mut().push(Rc::clone(&species));
        species
    }

    /// Creates a new SpeciesBuilder for constructing a Species with a fluent API.
    ///
    /// This method provides a builder pattern interface for creating and configuring
    /// a new Species within this model. The builder allows chaining method calls
    /// to set various properties of the Species before building it.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new species
    ///
    /// # Returns
    /// A SpeciesBuilder instance that can be used to configure and create the Species
    ///
    /// # Example
    /// ```no_run
    /// let species = model.build_species("glucose")
    ///     .set_name("Glucose")
    ///     .set_compartment("cytosol")
    ///     .set_initial_amount(10.0)
    ///     .build();
    /// ```
    pub fn build_species(&self, id: &str) -> SpeciesBuilder<'a> {
        SpeciesBuilder::new(self, id)
    }

    /// Returns a vector of all species in the model.
    ///
    /// # Returns
    /// A vector of all species in the model
    pub fn species(&self) -> Vec<Rc<Species<'a>>> {
        self.species.borrow().to_vec()
    }

    /// Retrieves a species from the model by its identifier.
    ///
    /// # Arguments
    /// * `id` - The identifier of the species to retrieve.
    ///
    /// # Returns
    /// An `Option<Rc<Species<'a>>>` which is `Some` if the species with the given ID exists,
    /// or `None` if it does not.
    pub fn get_species(&self, id: &str) -> Option<Rc<Species<'a>>> {
        self.species
            .borrow()
            .iter()
            .find(|species| (*species).id() == id)
            .map(|species| Rc::clone(species))
    }

    /// Creates a new Compartment within this model.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new compartment
    ///
    /// # Returns
    /// A new Compartment instance
    pub fn create_compartment(&self, id: &str) -> Rc<Compartment<'a>> {
        let compartment = Rc::new(Compartment::new(self, id));
        self.compartments.borrow_mut().push(Rc::clone(&compartment));
        compartment
    }

    /// Creates a new CompartmentBuilder for constructing a Compartment with a fluent API.
    ///
    /// This method provides a builder pattern interface for creating and configuring
    /// a new Compartment within this model. The builder allows chaining method calls
    /// to set various properties of the Compartment before building it.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new compartment
    ///
    /// # Returns
    /// A CompartmentBuilder instance that can be used to configure and create the Compartment
    ///
    /// # Example
    /// ```no_run
    /// let compartment = model.build_compartment("cytosol")
    ///     .set_name("Cytosol")
    ///     .build();
    /// ```
    pub fn build_compartment(&self, id: &str) -> CompartmentBuilder<'a> {
        CompartmentBuilder::new(self, id)
    }
}

impl<'a> Annotation for Model<'a> {
    /// Sets the annotation for the model.
    ///
    /// This function allows you to set a string annotation for the model,
    /// which can be used to provide additional information or metadata.
    ///
    /// # Arguments
    /// * `annotation` - A string slice that holds the annotation to set.
    fn set_annotation(&self, annotation: &str) {
        let_cxx_string!(annotation = annotation);
        self.model.borrow_mut().as_mut().setAnnotation1(&annotation);
    }

    /// Sets the annotation for the model using a serializable type.
    ///
    /// This function serializes the provided annotation into a string format
    /// and sets it as the model's annotation. It is useful for complex
    /// data structures that can be serialized.
    ///
    /// # Arguments
    /// * `annotation` - A reference to a serializable type that will be converted to a string.
    fn set_annotation_serde<T: Serialize>(&self, annotation: &T) {
        let annotation = to_string(annotation).unwrap();
        self.set_annotation(&annotation);
    }

    /// Gets the annotation for the model.
    ///
    /// # Returns
    /// The model's annotation as a String
    fn get_annotation(&self) -> String {
        let model_ptr: *mut sbmlcxx::Model =
            unsafe { self.model.borrow_mut().as_mut().get_unchecked_mut() as *mut _ };
        let annotation = unsafe { utils::getModelAnnotationString(model_ptr) };
        annotation.to_str().unwrap().to_string()
    }

    /// Gets the annotation for the model as a serializable type.
    ///
    /// This function deserializes the model's annotation from a string format
    /// into the specified type. It is useful for complex data structures that
    /// can be deserialized.
    ///
    /// # Returns
    /// The deserialized annotation as the specified type
    fn get_annotation_serde<T: for<'de> Deserialize<'de>>(&self) -> Result<T, DeError> {
        // Get the annotation string and attempt to parse it
        let annotation = self.get_annotation();
        let parsed: Wrapper<T> = from_str(&annotation)?;
        Ok(parsed.annotation)
    }
}
