//! This module provides a safe Rust interface to the libSBML Compartment class.
//!
//! The Compartment class represents a compartment in an SBML model.
//! It can represent a physical space, a cell, or any other entity that can contain species.
//! Compartments can have properties like size, volume, spatial dimensions, and units.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Compartment class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc};

use autocxx::c_uint;
use cxx::let_cxx_string;

use crate::{inner, into_id, model::Model, pin_ptr, sbmlcxx, sbo_term, upcast_annotation};

/// A safe wrapper around the libSBML Compartment class.
///
/// This struct maintains a reference to the underlying C++ Compartment object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
/// It provides methods to get and set various properties of the compartment like
/// size, volume, spatial dimensions, units, and more.
pub struct Compartment<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::Compartment>>,
}

// Set the inner trait for the Compartment struct
inner!(sbmlcxx::Compartment, Compartment<'a>);

// Set the annotation trait for the Compartment struct
upcast_annotation!(Compartment<'a>, sbmlcxx::Compartment, sbmlcxx::SBase);

// Set the into_id trait for the Compartment struct
into_id!(&Rc<Compartment<'_>>, id);

impl<'a> Compartment<'a> {
    /// Creates a new Compartment instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this compartment
    /// * `id` - The identifier for this compartment
    ///
    /// # Returns
    /// A new Compartment instance initialized with the given ID and added to the model
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let compartment_ptr = model.inner().borrow_mut().as_mut().createCompartment();
        let mut compartment = pin_ptr!(compartment_ptr, sbmlcxx::Compartment);

        // Set the id of the compartment
        let_cxx_string!(id = id);
        compartment.as_mut().setId(&id);

        Self {
            inner: RefCell::new(compartment),
        }
    }

    /// Creates a new Compartment instance from a unique pointer to a libSBML Compartment.
    ///
    /// This method is primarily used internally by the Model class to create
    /// Compartment instances from libSBML Compartment pointers.
    ///
    /// # Arguments
    /// * `ptr` - A unique pointer to a libSBML Compartment
    ///
    /// # Returns
    /// A new Compartment instance
    pub(crate) fn from_ptr(ptr: *mut sbmlcxx::Compartment) -> Self {
        let compartment = pin_ptr!(ptr, sbmlcxx::Compartment);
        Self {
            inner: RefCell::new(compartment),
        }
    }

    /// Gets the compartment's identifier.
    ///
    /// # Returns
    /// The compartment's ID as a String
    pub fn id(&self) -> String {
        self.inner.borrow().getId().to_str().unwrap().to_string()
    }

    /// Sets the compartment's identifier.
    ///
    /// # Arguments
    /// * `id` - The new identifier to set
    pub fn set_id(&self, id: &str) {
        let_cxx_string!(id = id);
        self.inner.borrow_mut().as_mut().setId(&id);
    }

    /// Gets the compartment's name.
    ///
    /// # Returns
    /// The compartment's name as a String
    pub fn name(&self) -> String {
        self.inner.borrow().getName().to_str().unwrap().to_string()
    }

    /// Sets the compartment's name.
    ///
    /// # Arguments
    /// * `name` - The new name to set
    pub fn set_name(&self, name: &str) {
        let_cxx_string!(name = name);
        self.inner.borrow_mut().as_mut().setName(&name);
    }

    /// Gets the spatial dimensions of the compartment.
    ///
    /// # Returns
    /// The number of spatial dimensions as a u32
    pub fn spatial_dimensions(&self) -> u32 {
        self.inner.borrow().getSpatialDimensions().0 as u32
    }

    /// Sets the spatial dimensions of the compartment.
    ///
    /// # Arguments
    /// * `spatial_dimensions` - The number of spatial dimensions to set (typically 0-3)
    pub fn set_spatial_dimensions(&self, spatial_dimensions: u32) {
        self.inner
            .borrow_mut()
            .as_mut()
            .setSpatialDimensions(c_uint::from(spatial_dimensions));
    }

    /// Gets the unit of measurement for the compartment.
    ///
    /// # Returns
    /// The unit of measurement as a String (e.g., "litre", "metre^3")
    pub fn unit(&self) -> String {
        self.inner.borrow().getUnits().to_str().unwrap().to_string()
    }

    /// Sets the unit of measurement for the compartment.
    ///
    /// # Arguments
    /// * `unit` - The unit of measurement to set (e.g., "litre", "metre^3")
    pub fn set_unit(&self, unit: &str) {
        let_cxx_string!(unit = unit);
        self.inner.borrow_mut().as_mut().setUnits(&unit);
    }

    /// Gets the size of the compartment.
    ///
    /// # Returns
    /// The size as a f64 in the units specified by the compartment's units attribute
    pub fn size(&self) -> f64 {
        self.inner.borrow().getSize()
    }

    /// Sets the size of the compartment.
    ///
    /// # Arguments
    /// * `size` - The size to set in the units specified by the compartment's units attribute
    pub fn set_size(&self, size: f64) {
        self.inner.borrow_mut().as_mut().setSize(size);
    }

    /// Gets the volume of the compartment.
    ///
    /// # Returns
    /// The volume as a f64 in the units specified by the compartment's units attribute
    pub fn volume(&self) -> f64 {
        self.inner.borrow().getVolume()
    }

    /// Sets the volume of the compartment.
    ///
    /// # Arguments
    /// * `volume` - The volume to set in the units specified by the compartment's units attribute
    pub fn set_volume(&self, volume: f64) {
        self.inner.borrow_mut().as_mut().setVolume(volume);
    }

    /// Gets whether the compartment is constant.
    ///
    /// # Returns
    /// true if the compartment's size is constant over time, false otherwise
    pub fn constant(&self) -> bool {
        self.inner.borrow().getConstant()
    }

    /// Sets whether the compartment is constant.
    ///
    /// # Arguments
    /// * `constant` - true if the compartment's size should be constant over time, false otherwise
    pub fn set_constant(&self, constant: bool) {
        self.inner.borrow_mut().as_mut().setConstant(constant);
    }

    /// Gets the outside compartment reference.
    ///
    /// # Returns
    /// The ID of the containing (outside) compartment as a String
    pub fn outside(&self) -> String {
        self.inner
            .borrow()
            .getOutside()
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Sets the outside compartment reference.
    ///
    /// # Arguments
    /// * `outside` - The ID of the containing (outside) compartment to set
    pub fn set_outside(&self, outside: &str) {
        let_cxx_string!(outside = outside);
        self.inner.borrow_mut().as_mut().setOutside(&outside);
    }

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::Compartment, sbmlcxx::SBase);
}

/// A builder for constructing Compartment instances with a fluent API.
///
/// This struct provides a builder pattern interface for creating and configuring
/// Compartment objects. It allows chaining method calls to set various properties
/// before finally constructing the Compartment.
pub struct CompartmentBuilder<'a> {
    compartment: Rc<Compartment<'a>>,
}

impl<'a> CompartmentBuilder<'a> {
    /// Creates a new CompartmentBuilder.
    ///
    /// # Arguments
    /// * `model` - The model that will contain the compartment
    /// * `id` - The identifier for the new compartment
    ///
    /// # Returns
    /// A new CompartmentBuilder instance
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let compartment = model.create_compartment(id);
        Self { compartment }
    }

    /// Sets the name of the compartment.
    ///
    /// # Arguments
    /// * `name` - The name to set
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn name(self, name: &str) -> Self {
        self.compartment.set_name(name);
        self
    }

    /// Sets the spatial dimensions of the compartment.
    ///
    /// # Arguments
    /// * `spatial_dimensions` - The number of spatial dimensions to set (typically 0-3)
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn spatial_dimensions(self, spatial_dimensions: u32) -> Self {
        self.compartment.set_spatial_dimensions(spatial_dimensions);
        self
    }

    /// Sets the unit of the compartment.
    ///
    /// # Arguments
    /// * `unit` - The unit to set (e.g., "litre", "metre^3")
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn unit(self, unit: &str) -> Self {
        self.compartment.set_unit(unit);
        self
    }

    /// Sets the size of the compartment.
    ///
    /// # Arguments
    /// * `size` - The size to set in the units specified by the compartment's units attribute
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn size(self, size: f64) -> Self {
        self.compartment.set_size(size);
        self
    }

    /// Sets the volume of the compartment.
    ///
    /// # Arguments
    /// * `volume` - The volume to set in the units specified by the compartment's units attribute
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn volume(self, volume: f64) -> Self {
        self.compartment.set_volume(volume);
        self
    }

    /// Sets the outside compartment reference.
    ///
    /// # Arguments
    /// * `outside` - The ID of the containing (outside) compartment to set
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn outside(self, outside: &str) -> Self {
        self.compartment.set_outside(outside);
        self
    }

    /// Sets whether the compartment is constant.
    ///
    /// # Arguments
    /// * `constant` - true if the compartment's size should be constant over time
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn constant(self, constant: bool) -> Self {
        self.compartment.set_constant(constant);
        self
    }

    /// Sets the annotation for this compartment.
    ///
    /// # Arguments
    /// * `annotation` - The XML annotation string to set
    ///
    /// # Returns
    /// Result containing the builder instance or an error if the annotation is invalid
    pub fn annotation(self, annotation: &str) -> Result<Self, SeError> {
        self.compartment
            .set_annotation(annotation)
            .map_err(|e| SeError::Custom(e.to_string()))?;
        Ok(self)
    }

    /// Sets a serializable annotation for this compartment.
    ///
    /// # Arguments
    /// * `annotation` - The annotation to serialize to XML and set
    ///
    /// # Returns
    /// Result containing the builder instance or a serialization error
    pub fn annotation_serde<T: Serialize>(self, annotation: &T) -> Result<Self, SeError> {
        let annotation = to_string(annotation)?;
        self.compartment
            .set_annotation(&annotation)
            .map_err(|e| SeError::Custom(e.to_string()))?;
        Ok(self)
    }

    /// Builds and returns the configured Compartment instance.
    ///
    /// # Returns
    /// The fully configured Compartment wrapped in an Rc
    pub fn build(self) -> Rc<Compartment<'a>> {
        self.compartment
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_compartment_new() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let compartment = Compartment::new(&model, "test");

        // Use all setters to set all properties
        compartment.set_name("test");
        compartment.set_spatial_dimensions(3);
        compartment.set_unit("test");
        compartment.set_size(1.0);
        compartment.set_volume(1.0);
        compartment.set_outside("test");
        compartment.set_constant(true);

        assert_eq!(compartment.id(), "test");
        assert_eq!(compartment.name(), "test");
        assert_eq!(compartment.spatial_dimensions(), 3);
        assert_eq!(compartment.unit(), "test");
        assert_eq!(compartment.size(), 1.0);
        assert_eq!(compartment.volume(), 1.0);
        assert_eq!(compartment.outside(), "test");
        assert_eq!(compartment.constant(), true);
    }

    #[test]
    fn test_compartment_builder() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let compartment = CompartmentBuilder::new(&model, "test")
            .name("test")
            .spatial_dimensions(3)
            .unit("test")
            .size(1.0)
            .volume(1.0)
            .outside("test")
            .constant(true)
            .build();

        assert_eq!(compartment.id(), "test");
        assert_eq!(compartment.name(), "test");
        assert_eq!(compartment.spatial_dimensions(), 3);
        assert_eq!(compartment.unit(), "test");
        assert_eq!(compartment.size(), 1.0);
        assert_eq!(compartment.volume(), 1.0);
        assert_eq!(compartment.outside(), "test");
        assert_eq!(compartment.constant(), true);
    }

    #[test]
    fn test_compartment_annotation() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let compartment = CompartmentBuilder::new(&model, "test")
            .annotation("<test>test</test>")
            .unwrap()
            .build();

        assert_eq!(
            compartment
                .get_annotation()
                .replace("\n", "")
                .replace(" ", ""),
            "<annotation><test>test</test></annotation>"
        );
    }

    #[test]
    fn test_compartment_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let annotation = TestAnnotation {
            test: "test".to_string(),
        };

        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let compartment = CompartmentBuilder::new(&model, "test")
            .annotation_serde(&annotation)
            .unwrap()
            .build();

        let extracted: TestAnnotation = compartment.get_annotation_serde().unwrap();

        assert_eq!(extracted.test, "test");
    }
}
