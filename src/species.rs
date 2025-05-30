//! This module provides a safe Rust interface to the libSBML Species class.
//!
//! The Species class represents a chemical or biological entity in an SBML model.
//! It can represent molecules, ions, proteins, or any other entity that participates
//! in reactions. Each species can have properties like initial amount/concentration,
//! boundary conditions, and compartment location.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Species class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc};

use cxx::let_cxx_string;

use crate::{
    clone, get_unit_definition, inner, into_id,
    model::Model,
    optional_property, pin_ptr,
    prelude::IntoId,
    required_property, sbase,
    sbmlcxx::{self},
    sbo_term,
    traits::{fromptr::FromPtr, sbase::SBase},
    upcast_annotation,
};

/// A safe wrapper around the libSBML Species class.
///
/// This struct maintains a reference to the underlying C++ Species object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct Species<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::Species>>,
}

// Set the inner trait for the Species struct
inner!(sbmlcxx::Species, Species<'a>);

// Set the sbase trait for the Species struct
sbase!(Species<'a>, sbmlcxx::Species);

// Set the annotation trait for the Species struct
upcast_annotation!(Species<'a>, sbmlcxx::Species, sbmlcxx::SBase);

// Implement the Clone trait for the Species struct
clone!(Species<'a>, sbmlcxx::Species);

// Set the into_id trait for the Species struct
into_id!(&Rc<Species<'_>>, id);

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
        let mut species = pin_ptr!(species_ptr, sbmlcxx::Species);

        // Set the default values for the species
        species.as_mut().initDefaults();

        // Set the id of the species
        let_cxx_string!(id = id);
        species.as_mut().setId(&id);

        Self {
            inner: RefCell::new(species),
        }
    }

    /// Returns a reference to the inner RefCell containing the Species pointer.
    ///
    /// This is primarily used internally by other parts of the library.
    #[allow(dead_code)]
    pub(crate) fn inner(&self) -> &RefCell<Pin<&'a mut sbmlcxx::Species>> {
        &self.inner
    }

    // Setter and getter for id
    required_property!(Species<'a>, id, String, getId, setId);

    // Setter and getter for name
    optional_property!(Species<'a>, name, String, getName, setName, isSetName);

    // Setter and getter for compartment
    optional_property!(
        Species<'a>,
        compartment,
        String,
        getCompartment,
        setCompartment,
        isSetCompartment,
        impl IntoId
    );

    // Setter and getter for initial amount
    optional_property!(
        Species<'a>,
        initial_amount,
        f64,
        getInitialAmount,
        setInitialAmount,
        isSetInitialAmount
    );

    // Setter and getter for initial concentration
    optional_property!(
        Species<'a>,
        initial_concentration,
        f64,
        getInitialConcentration,
        setInitialConcentration,
        isSetInitialConcentration
    );

    // Setter and getter for unit
    optional_property!(Species<'a>, unit, String, getUnits, setUnits, isSetUnits);

    // Setter and getter for boundary condition
    optional_property!(
        Species<'a>,
        boundary_condition,
        bool,
        getBoundaryCondition,
        setBoundaryCondition,
        isSetBoundaryCondition
    );

    // Setter and getter for constant
    required_property!(Species<'a>, constant, bool, getConstant, setConstant);

    // Setter and getter for has only substance units
    optional_property!(
        Species<'a>,
        has_only_substance_units,
        bool,
        getHasOnlySubstanceUnits,
        setHasOnlySubstanceUnits,
        isSetHasOnlySubstanceUnits
    );

    optional_property!(
        Species<'a>,
        units,
        String,
        getUnits,
        setUnits,
        isSetUnits,
        impl IntoId
    );

    // Gets the unit definition for the species
    get_unit_definition!(units);

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::Species, sbmlcxx::SBase);
}

impl FromPtr<sbmlcxx::Species> for Species<'_> {
    /// Creates a new Species instance from a unique pointer to a libSBML Species.
    ///
    /// This method is primarily used internally by the Model class to create
    /// Species instances from libSBML Species pointers.
    ///
    /// # Arguments
    /// * `ptr` - A unique pointer to a libSBML Species
    fn from_ptr(ptr: *mut sbmlcxx::Species) -> Self {
        let species = pin_ptr!(ptr, sbmlcxx::Species);
        Self {
            inner: RefCell::new(species),
        }
    }
}

/// A builder for constructing Species instances with a fluent API.
///
/// This struct provides a builder pattern interface for creating and configuring
/// Species objects. It allows chaining method calls to set various properties
/// before finally constructing the Species.
///
/// # Example
/// ```no_run
/// use sbml::prelude::*;
///
/// let doc = SBMLDocument::default();
/// let model = Model::new(&doc, "test");
/// let species = model.build_species("glucose")
///     .name("Glucose")
///     .compartment("cytosol")
///     .initial_amount(10.0)
///     .build();
/// ```
pub struct SpeciesBuilder<'a> {
    species: Rc<Species<'a>>,
}

impl<'a> SpeciesBuilder<'a> {
    /// Creates a new SpeciesBuilder.
    ///
    /// # Arguments
    /// * `model` - The model that will contain the species
    /// * `id` - The identifier for the new species
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let species = model.create_species(id);
        Self { species }
    }

    /// Sets the name of the species.
    ///
    /// # Arguments
    /// * `name` - The name to set
    pub fn name(self, name: &str) -> Self {
        self.species.set_name(name);
        self
    }

    /// Sets the compartment that contains this species.
    ///
    /// # Arguments
    /// * `compartment` - The compartment identifier
    pub fn compartment(self, compartment: impl IntoId) -> Self {
        self.species.set_compartment(compartment);
        self
    }

    /// Sets the initial concentration of the species.
    ///
    /// # Arguments
    /// * `concentration` - The initial concentration value
    pub fn initial_concentration(self, concentration: f64) -> Self {
        self.species.set_initial_concentration(concentration);
        self
    }

    /// Sets the initial amount of the species.
    ///
    /// # Arguments
    /// * `amount` - The initial amount value
    pub fn initial_amount(self, amount: f64) -> Self {
        self.species.set_initial_amount(amount);
        self
    }

    /// Sets the unit of the species.
    ///
    /// # Arguments
    /// * `unit` - The unit to set
    pub fn unit(self, unit: impl IntoId) -> Self {
        self.species.set_unit(unit.into_id());
        self
    }
    /// Sets whether this species has a boundary condition.
    ///
    /// # Arguments
    /// * `boundary` - True if this is a boundary species
    pub fn boundary_condition(self, boundary: bool) -> Self {
        self.species.set_boundary_condition(boundary);
        self
    }

    /// Sets whether this species is constant.
    ///
    /// # Arguments
    /// * `constant` - True if this species should be constant
    pub fn constant(self, constant: bool) -> Self {
        self.species.set_constant(constant);
        self
    }

    /// Sets whether this species has only substance units.
    ///
    /// # Arguments
    /// * `has_only_substance_units` - True if this species has only substance units
    pub fn has_only_substance_units(self, has_only_substance_units: bool) -> Self {
        self.species
            .set_has_only_substance_units(has_only_substance_units);
        self
    }

    /// Sets the annotation for this species.
    ///
    /// # Arguments
    /// * `annotation` - The annotation string to set
    pub fn annotation(self, annotation: &str) -> Result<Self, SeError> {
        self.species
            .set_annotation(annotation)
            .map_err(|e| SeError::Custom(e.to_string()))?;
        Ok(self)
    }

    /// Sets a serializable annotation for this species.
    ///
    /// # Arguments
    /// * `annotation` - The annotation to serialize and set
    ///
    /// # Returns
    /// Self with Result indicating success or serialization error
    pub fn annotation_serde<T: Serialize>(self, annotation: &T) -> Result<Self, SeError> {
        let annotation = to_string(annotation)?;
        self.species
            .set_annotation(&annotation)
            .map_err(|e| SeError::Custom(e.to_string()))?;
        Ok(self)
    }

    /// Builds and returns the configured Species instance.
    ///
    /// # Note
    /// If annotation_serde() was used in the builder chain, this should be called
    /// with build()? to handle potential serialization errors.
    pub fn build(self) -> Rc<Species<'a>> {
        self.species
    }
}

impl<'a> std::fmt::Debug for Species<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Species");
        ds.field("id", &self.id());
        ds.field("name", &self.name());
        ds.field("compartment", &self.compartment());
        ds.field("initial_amount", &self.initial_amount());
        ds.field("initial_concentration", &self.initial_concentration());
        ds.field("unit", &self.unit());
        ds.field("boundary_condition", &self.boundary_condition());
        ds.field("constant", &self.constant());
        ds.field("has_only_substance_units", &self.has_only_substance_units());
        ds.finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::{unit::UnitKind, SBMLDocument};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_species_from_model() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let species = Species::new(&model, "glucose");
        species.set_name("Glucose");
        species.set_compartment("cytosol");
        species.set_initial_amount(1.0);
        species.set_boundary_condition(true);
        species.set_constant(false);
        species.set_has_only_substance_units(true);
        species.set_unit("mole");

        assert_eq!(species.name(), Some("Glucose".to_string()));
        assert_eq!(species.compartment(), Some("cytosol".to_string()));
        assert_eq!(species.initial_amount(), Some(1.0));
        assert_eq!(species.boundary_condition(), Some(true));
        assert!(!species.constant());
        assert_eq!(species.has_only_substance_units(), Some(true));
        assert_eq!(species.unit(), Some("mole".to_string()));
    }

    #[test]
    fn test_species_builder() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let builder = SpeciesBuilder::new(&model, "glucose");
        let species = builder
            .name("Glucose")
            .compartment("cytosol")
            .initial_amount(1.0)
            .unit("mole")
            .boundary_condition(true)
            .constant(false)
            .has_only_substance_units(true)
            .build();

        assert_eq!(species.name(), Some("Glucose".to_string()));
        assert_eq!(species.id(), "glucose");
        assert_eq!(species.compartment(), Some("cytosol".to_string()));
        assert_eq!(species.initial_amount(), Some(1.0));
        assert_eq!(species.boundary_condition(), Some(true));
        assert!(!species.constant());
        assert_eq!(species.has_only_substance_units(), Some(true));
        assert_eq!(species.unit(), Some("mole".to_string()));
    }

    #[test]
    fn test_species_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let species = Species::new(&model, "glucose");
        species.set_annotation("<test>1</test>").unwrap();
        assert_eq!(
            species.get_annotation().replace("\n", "").replace(' ', ""),
            "<annotation><test>1</test></annotation>"
        );
    }

    #[test]
    fn test_species_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct Test {
            test: String,
        }

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let species = Species::new(&model, "glucose");
        species
            .set_annotation_serde(&Test {
                test: "test".to_string(),
            })
            .unwrap();
        assert_eq!(species.get_annotation_serde::<Test>().unwrap().test, "test");
    }

    #[test]
    fn test_species_unit_definition() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        // Create the unit definition
        model
            .build_unit_definition("ml", "milliliter")
            .unit(UnitKind::Litre, Some(-1), Some(-3), None, None)
            .build();

        model
            .build_unit_definition("M", "Molar")
            .unit(UnitKind::Mole, Some(1), Some(1), None, None)
            .unit(UnitKind::Litre, Some(-1), Some(1), None, None)
            .build();

        model
            .build_compartment("compartment")
            .unit("ml")
            .constant(true)
            .build();

        let species = model
            .build_species("species")
            .compartment("compartment")
            .unit("M")
            .constant(true)
            .build();

        let valid = doc.check_consistency();

        if !valid.valid {
            println!("{:#?}", valid.errors);
            panic!("Invalid SBML document");
        }

        let unit_definition = species.unit_definition().unwrap();
        assert_eq!(unit_definition.id(), "M");
        assert_eq!(unit_definition.units().len(), 2);

        // Mole
        assert_eq!(unit_definition.units()[0].kind(), UnitKind::Mole);
        assert_eq!(unit_definition.units()[0].exponent(), 1);
        assert_eq!(unit_definition.units()[0].scale(), 1);
        assert_eq!(unit_definition.units()[0].multiplier(), 1.0);
        assert_eq!(unit_definition.units()[0].offset(), 0.0);

        // Litre
        assert_eq!(unit_definition.units()[1].kind(), UnitKind::Litre);
        assert_eq!(unit_definition.units()[1].exponent(), -1);
        assert_eq!(unit_definition.units()[1].scale(), 1);
        assert_eq!(unit_definition.units()[1].multiplier(), 1.0);
        assert_eq!(unit_definition.units()[1].offset(), 0.0);
    }
}
