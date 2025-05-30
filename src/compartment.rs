//! This module provides a safe Rust interface to the libSBML Compartment class.
//!
//! The Compartment class represents a compartment in an SBML model.
//! It can represent a physical space, a cell, or any other entity that can contain species.
//! Compartments can have properties like size, volume, spatial dimensions, and units.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Compartment class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc};

use cxx::let_cxx_string;

use crate::{
    clone, get_unit_definition, inner, into_id,
    model::Model,
    optional_property, pin_ptr, required_property, sbase, sbmlcxx, sbo_term,
    traits::{fromptr::FromPtr, intoid::IntoId, sbase::SBase},
    upcast_annotation,
};

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

// Set the sbase trait for the Compartment struct
sbase!(Compartment<'a>, sbmlcxx::Compartment);

// Set the annotation trait for the Compartment struct
upcast_annotation!(Compartment<'a>, sbmlcxx::Compartment, sbmlcxx::SBase);

// Set the into_id trait for the Compartment struct
into_id!(&Rc<Compartment<'_>>, id);

// Implement the Clone trait for the Compartment struct
clone!(Compartment<'a>, sbmlcxx::Compartment);

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

    // Gets the entire unit definition for the compartment.
    get_unit_definition!(unit);

    // Getter and setter methods for the id property
    required_property!(Compartment<'a>, id, String, getId, setId);

    // Getter and setter methods for the name property
    optional_property!(Compartment<'a>, name, String, getName, setName, isSetName);

    // Getter and setter methods for the spatial dimensions property
    optional_property!(
        Compartment<'a>,
        spatial_dimensions,
        u32,
        getSpatialDimensions,
        setSpatialDimensions,
        isSetSpatialDimensions
    );

    // Getter and setter methods for the unit property
    optional_property!(
        Compartment<'a>,
        unit,
        String,
        getUnits,
        setUnits,
        isSetUnits,
        impl IntoId
    );

    // Getter and setter methods for the size property
    optional_property!(Compartment<'a>, size, f64, getSize, setSize, isSetSize);

    // Getter and setter methods for the volume property
    optional_property!(
        Compartment<'a>,
        volume,
        f64,
        getVolume,
        setVolume,
        isSetVolume
    );

    // Getter and setter methods for the constant property
    optional_property!(
        Compartment<'a>,
        constant,
        bool,
        getConstant,
        setConstant,
        isSetConstant
    );

    // Getter and setter methods for the outside property
    optional_property!(
        Compartment<'a>,
        outside,
        String,
        getOutside,
        setOutside,
        isSetOutside
    );

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::Compartment, sbmlcxx::SBase);
}

impl FromPtr<sbmlcxx::Compartment> for Compartment<'_> {
    fn from_ptr(ptr: *mut sbmlcxx::Compartment) -> Self {
        let compartment = pin_ptr!(ptr, sbmlcxx::Compartment);
        Self {
            inner: RefCell::new(compartment),
        }
    }
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
    pub fn unit(self, unit: impl IntoId) -> Self {
        self.compartment.set_unit(unit.into_id());
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

impl std::fmt::Debug for Compartment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Compartment");
        ds.field("id", &self.id());
        ds.field("name", &self.name());
        ds.field("spatial_dimensions", &self.spatial_dimensions());
        ds.field("unit", &self.unit());
        ds.field("size", &self.size());
        ds.field("volume", &self.volume());
        ds.field("outside", &self.outside());
        ds.field("constant", &self.constant());
        ds.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_compartment_new() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let compartment = Compartment::new(&model, "test");

        // Use all setters to set all properties
        compartment.set_name("test");
        compartment.set_spatial_dimensions(3u32);
        compartment.set_unit("test");
        compartment.set_size(1.0);
        compartment.set_volume(1.0);
        compartment.set_outside("test");
        compartment.set_constant(true);

        assert_eq!(compartment.id(), "test");
        assert_eq!(compartment.name(), Some("test".to_string()));
        assert_eq!(compartment.spatial_dimensions(), Some(3));
        assert_eq!(compartment.unit(), Some("test".to_string()));
        assert_eq!(compartment.size(), Some(1.0));
        assert_eq!(compartment.volume(), Some(1.0));
        assert_eq!(compartment.outside(), Some("test".to_string()));
        assert_eq!(compartment.constant(), Some(true));
    }

    #[test]
    fn test_compartment_builder() {
        let doc = SBMLDocument::default();
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
        assert_eq!(compartment.name(), Some("test".to_string()));
        assert_eq!(compartment.spatial_dimensions(), Some(3));
        assert_eq!(compartment.unit(), Some("test".to_string()));
        assert_eq!(compartment.size(), Some(1.0));
        assert_eq!(compartment.volume(), Some(1.0));
        assert_eq!(compartment.outside(), Some("test".to_string()));
        assert_eq!(compartment.constant(), Some(true));
    }

    #[test]
    fn test_compartment_annotation() {
        let doc = SBMLDocument::default();
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

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let compartment = CompartmentBuilder::new(&model, "test")
            .annotation_serde(&annotation)
            .unwrap()
            .build();

        let extracted: TestAnnotation = compartment.get_annotation_serde().unwrap();

        assert_eq!(extracted.test, "test");
    }

    #[test]
    fn test_compartment_unit_definition() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");
        model
            .build_unit_definition("ml", "milliliter")
            .unit(UnitKind::Litre, Some(1), Some(-3), None, None)
            .build();

        let compartment = CompartmentBuilder::new(&model, "compartment")
            .unit("ml")
            .constant(true)
            .build();

        assert!(doc.check_consistency().valid);

        let unit_definition = compartment.unit_definition().unwrap();
        assert_eq!(unit_definition.id(), "ml");
        assert_eq!(unit_definition.units().len(), 1);
        assert_eq!(unit_definition.units()[0].kind(), UnitKind::Litre);
        assert_eq!(unit_definition.units()[0].exponent(), 1);
        assert_eq!(unit_definition.units()[0].scale(), -3);
        assert_eq!(unit_definition.units()[0].multiplier(), 1.0);
        assert_eq!(unit_definition.units()[0].offset(), 0.0);
    }
}
