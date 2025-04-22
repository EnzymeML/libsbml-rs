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
    clone, inner, into_id,
    model::Model,
    pin_ptr,
    prelude::IntoId,
    sbmlcxx::{self},
    sbo_term,
    traits::fromptr::FromPtr,
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

    /// Gets the species' identifier.
    ///
    /// # Returns
    /// The species' ID as a String
    pub fn id(&self) -> String {
        self.inner.borrow().getId().to_str().unwrap().to_string()
    }

    /// Sets the species' identifier.
    ///
    /// # Arguments
    /// * `id` - The new identifier to set
    pub fn set_id(&self, id: &str) {
        let_cxx_string!(id = id);
        self.inner.borrow_mut().as_mut().setId(&id);
    }

    /// Gets the species' name.
    ///
    /// # Returns
    /// The species' name as a String
    pub fn name(&self) -> String {
        self.inner.borrow().getName().to_str().unwrap().to_string()
    }

    /// Sets the species' name.
    ///
    /// # Arguments
    /// * `name` - The new name to set
    pub fn set_name(&self, name: &str) {
        let_cxx_string!(name = name);
        self.inner.borrow_mut().as_mut().setName(&name);
    }

    /// Gets the compartment where this species is located.
    ///
    /// # Returns
    /// The compartment identifier as a String
    pub fn compartment(&self) -> String {
        self.inner
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
    pub fn set_compartment(&self, compartment: impl IntoId<'a>) {
        let compartment = compartment.into_id();
        let_cxx_string!(compartment = compartment);
        self.inner
            .borrow_mut()
            .as_mut()
            .setCompartment(&compartment);
    }

    /// Gets the initial amount of this species.
    ///
    /// # Returns
    /// The initial amount as a f64
    pub fn initial_amount(&self) -> f64 {
        self.inner.borrow().getInitialAmount()
    }

    /// Sets the initial amount of this species.
    ///
    /// # Arguments
    /// * `initial_amount` - The initial amount to set
    pub fn set_initial_amount(&self, initial_amount: f64) {
        self.inner
            .borrow_mut()
            .as_mut()
            .setInitialAmount(initial_amount);
    }

    /// Gets the initial concentration of this species.
    ///
    /// # Returns
    /// The initial concentration as a f64
    pub fn initial_concentration(&self) -> f64 {
        self.inner.borrow().getInitialConcentration()
    }

    /// Sets the initial concentration of this species.
    ///
    /// # Arguments
    /// * `initial_concentration` - The initial concentration to set
    pub fn set_initial_concentration(&self, initial_concentration: f64) {
        self.inner
            .borrow_mut()
            .as_mut()
            .setInitialConcentration(initial_concentration);
    }

    /// Gets the unit of this species.
    ///
    /// # Returns
    /// The unit of the species
    pub fn unit(&self) -> String {
        self.inner.borrow().getUnits().to_str().unwrap().to_string()
    }

    /// Sets the unit of this species.
    ///
    /// # Arguments
    /// * `unit` - The unit to set
    pub fn set_unit(&self, unit: &str) {
        let_cxx_string!(unit = unit);
        self.inner.borrow_mut().as_mut().setUnits(&unit);
    }

    /// Gets whether this species has a boundary condition.
    ///
    /// A boundary species is one whose value is not determined by any rule or reaction
    /// in the model but is set by some external mechanism.
    ///
    /// # Returns
    /// true if this species has a boundary condition, false otherwise
    pub fn boundary_condition(&self) -> bool {
        self.inner.borrow().getBoundaryCondition()
    }

    /// Sets whether this species has a boundary condition.
    ///
    /// # Arguments
    /// * `boundary_condition` - Whether this species should have a boundary condition
    pub fn set_boundary_condition(&self, boundary_condition: bool) {
        self.inner
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
        self.inner.borrow().getConstant()
    }

    /// Sets whether this species is constant.
    ///
    /// # Arguments
    /// * `constant` - Whether this species should be constant
    pub fn set_constant(&self, constant: bool) {
        self.inner.borrow_mut().as_mut().setConstant(constant);
    }

    /// Gets whether this species has only substance units.
    ///
    /// If true, the units of the species' amount is interpreted as substance units only,
    /// rather than substance/size units.
    ///
    /// # Returns
    /// true if this species has only substance units, false otherwise
    pub fn has_only_substance_units(&self) -> bool {
        self.inner.borrow().getHasOnlySubstanceUnits()
    }

    /// Sets whether this species has only substance units.
    ///
    /// # Arguments
    /// * `has_only_substance_units` - Whether this species should have only substance units
    pub fn set_has_only_substance_units(&self, has_only_substance_units: bool) {
        self.inner
            .borrow_mut()
            .as_mut()
            .setHasOnlySubstanceUnits(has_only_substance_units);
    }

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
/// let doc = SBMLDocument::new(3, 2);
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
    pub fn compartment(self, compartment: impl IntoId<'a>) -> Self {
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
    pub fn unit(self, unit: &str) -> Self {
        self.species.set_unit(unit);
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

#[cfg(test)]
mod tests {
    use crate::SBMLDocument;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_species_from_model() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let species = Species::new(&model, "glucose");
        species.set_name("Glucose");
        species.set_compartment("cytosol");
        species.set_initial_amount(1.0);
        species.set_boundary_condition(true);
        species.set_constant(false);
        species.set_has_only_substance_units(true);
        species.set_unit("mole");

        assert_eq!(species.name(), "Glucose");
        assert_eq!(species.compartment(), "cytosol");
        assert_eq!(species.initial_amount(), 1.0);
        assert_eq!(species.boundary_condition(), true);
        assert_eq!(species.constant(), false);
        assert_eq!(species.has_only_substance_units(), true);
        assert_eq!(species.unit(), "mole");
    }

    #[test]
    fn test_species_builder() {
        let doc = SBMLDocument::new(3, 2);
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

        assert_eq!(species.name(), "Glucose");
        assert_eq!(species.id(), "glucose");
        assert_eq!(species.compartment(), "cytosol");
        assert_eq!(species.initial_amount(), 1.0);
        assert_eq!(species.boundary_condition(), true);
        assert_eq!(species.constant(), false);
        assert_eq!(species.has_only_substance_units(), true);
        assert_eq!(species.unit(), "mole");
    }

    #[test]
    fn test_species_annotation() {
        let doc = SBMLDocument::new(3, 2);
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

        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let species = Species::new(&model, "glucose");
        species
            .set_annotation_serde(&Test {
                test: "test".to_string(),
            })
            .unwrap();
        assert_eq!(species.get_annotation_serde::<Test>().unwrap().test, "test");
    }
}
