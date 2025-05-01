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

use crate::{
    clone,
    collections::*,
    compartment::{Compartment, CompartmentBuilder},
    inner,
    parameter::{Parameter, ParameterBuilder},
    pin_ptr,
    prelude::IntoId,
    reaction::{Reaction, ReactionBuilder},
    rule::{AssignmentRuleBuilder, RateRuleBuilder, Rule, RuleType},
    sbmlcxx::{self},
    sbmldoc::SBMLDocument,
    sbo_term, set_collection_annotation,
    species::{Species, SpeciesBuilder},
    traits::fromptr::FromPtr,
    unitdef::{UnitDefinition, UnitDefinitionBuilder},
    upcast_annotation,
};

/// A safe wrapper around the libSBML Model class.
///
/// This struct maintains a reference to the underlying C++ Model object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
///
/// The Model class represents a complete biological model, containing:
/// - Species (chemical species/molecules)
/// - Compartments (physical containers)
/// - Reactions (transformations between species)
/// - Unit definitions (custom units)
/// - Other SBML elements
pub struct Model<'a> {
    /// The underlying libSBML Model pointer wrapped in RefCell and Pin
    inner: RefCell<Pin<&'a mut sbmlcxx::Model>>,
    /// List of all Species in the model
    list_of_species: RefCell<Vec<Rc<Species<'a>>>>,
    /// List of all Compartments in the model  
    list_of_compartments: RefCell<Vec<Rc<Compartment<'a>>>>,
    /// List of all UnitDefinitions in the model
    list_of_unit_definitions: RefCell<Vec<Rc<UnitDefinition<'a>>>>,
    /// List of all Reactions in the model
    list_of_reactions: RefCell<Vec<Rc<Reaction<'a>>>>,
    /// List of all Parameters in the model
    list_of_parameters: RefCell<Vec<Rc<Parameter<'a>>>>,
    /// List of all RateRules in the model
    list_of_rate_rules: RefCell<Vec<Rc<Rule<'a>>>>,
    /// List of all AssignmentRules in the model
    list_of_assignment_rules: RefCell<Vec<Rc<Rule<'a>>>>,
}

// Set the inner trait for the Model struct
inner!(sbmlcxx::Model, Model<'a>);

// Set the annotation trait for the Model struct
upcast_annotation!(Model<'a>, sbmlcxx::Model, sbmlcxx::SBase);

// Implement the Clone trait for the Model struct
clone!(
    Model<'a>,
    sbmlcxx::Model,
    list_of_species,
    list_of_compartments,
    list_of_unit_definitions,
    list_of_reactions,
    list_of_parameters,
    list_of_rate_rules,
    list_of_assignment_rules
);

impl<'a> Model<'a> {
    /// Creates a new Model instance within the given SBMLDocument.
    ///
    /// # Arguments
    /// * `document` - The parent SBMLDocument that will contain this model
    /// * `id` - The identifier for this model
    ///
    /// # Returns
    /// A new Model instance initialized with the given ID and empty lists of components
    pub fn new(document: &SBMLDocument, id: &str) -> Self {
        let model_ptr = document.inner().borrow_mut().pin_mut().createModel(id);
        let model = pin_ptr!(model_ptr, sbmlcxx::Model);

        Self {
            inner: RefCell::new(model),
            list_of_species: RefCell::new(Vec::new()),
            list_of_compartments: RefCell::new(Vec::new()),
            list_of_unit_definitions: RefCell::new(Vec::new()),
            list_of_reactions: RefCell::new(Vec::new()),
            list_of_parameters: RefCell::new(Vec::new()),
            list_of_rate_rules: RefCell::new(Vec::new()),
            list_of_assignment_rules: RefCell::new(Vec::new()),
        }
    }

    /// Returns a reference to the inner RefCell containing the Model pointer.
    ///
    /// This is primarily used internally by other parts of the library to access
    /// the underlying libSBML Model object.
    pub(crate) fn inner(&self) -> &RefCell<Pin<&'a mut sbmlcxx::Model>> {
        &self.inner
    }

    /// Gets the model's identifier.
    ///
    /// # Returns
    /// The model's ID as a String
    pub fn id(&self) -> String {
        self.inner.borrow().getId().to_str().unwrap().to_string()
    }

    /// Sets the model's identifier.
    ///
    /// # Arguments
    /// * `id` - The new identifier to set for the model
    pub fn set_id(&self, id: &str) {
        let_cxx_string!(id = id);
        self.inner.borrow_mut().as_mut().setId(&id);
    }

    /// Gets the model's name.
    ///
    /// # Returns
    /// The model's name as a String
    pub fn name(&self) -> String {
        self.inner.borrow().getName().to_str().unwrap().to_string()
    }

    /// Sets the model's name.
    ///
    /// # Arguments
    /// * `name` - The new name to set for the model
    pub fn set_name(&self, name: &str) {
        let_cxx_string!(name = name);
        self.inner.borrow_mut().as_mut().setName(&name);
    }

    /// Creates a new Species within this model.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new species
    ///
    /// # Returns
    /// A new Species instance wrapped in an Rc
    pub fn create_species(&self, id: &str) -> Rc<Species<'a>> {
        let species = Rc::new(Species::new(self, id));
        self.list_of_species.borrow_mut().push(Rc::clone(&species));
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
    pub fn build_species(&self, id: &str) -> SpeciesBuilder<'a> {
        SpeciesBuilder::new(self, id)
    }

    /// Returns a vector of all species in the model.
    ///
    /// # Returns
    /// A vector containing Rc references to all Species in the model
    pub fn list_of_species(&self) -> Vec<Rc<Species<'a>>> {
        self.list_of_species.borrow().to_vec()
    }

    /// Retrieves a species from the model by its identifier.
    ///
    /// # Arguments
    /// * `id` - The identifier of the species to retrieve
    ///
    /// # Returns
    /// Some(`Rc<Species>`) if found, None if not found
    pub fn get_species(&self, id: &str) -> Option<Rc<Species<'a>>> {
        self.list_of_species
            .borrow()
            .iter()
            .find(|species| (*species).id() == id)
            .map(Rc::clone)
    }

    /// Creates a new Compartment within this model.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new compartment
    ///
    /// # Returns
    /// A new Compartment instance wrapped in an Rc
    pub fn create_compartment(&self, id: &str) -> Rc<Compartment<'a>> {
        let compartment = Rc::new(Compartment::new(self, id));
        self.list_of_compartments
            .borrow_mut()
            .push(Rc::clone(&compartment));
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
    pub fn build_compartment(&self, id: &str) -> CompartmentBuilder<'a> {
        CompartmentBuilder::new(self, id)
    }

    /// Returns a vector of all compartments in the model.
    ///
    /// # Returns
    /// A vector containing Rc references to all Compartments in the model
    pub fn list_of_compartments(&self) -> Vec<Rc<Compartment<'a>>> {
        self.list_of_compartments.borrow().to_vec()
    }

    /// Retrieves a compartment from the model by its identifier.
    ///
    /// # Arguments
    /// * `id` - The identifier of the compartment to retrieve
    ///
    /// # Returns
    /// Some(`Rc<Compartment>`) if found, None if not found
    pub fn get_compartment(&self, id: &str) -> Option<Rc<Compartment<'a>>> {
        self.list_of_compartments
            .borrow()
            .iter()
            .find(|compartment| (*compartment).id() == id)
            .map(Rc::clone)
    }

    /// Creates a new UnitDefinition within this model.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new unit definition
    /// * `name` - The name of the unit definition
    ///
    /// # Returns
    /// A new UnitDefinition instance wrapped in an Rc
    pub fn create_unit_definition(&self, id: &str, name: &str) -> Rc<UnitDefinition<'a>> {
        let unit_definition = Rc::new(UnitDefinition::new(self, id, name));
        self.list_of_unit_definitions
            .borrow_mut()
            .push(Rc::clone(&unit_definition));
        unit_definition
    }

    /// Creates a new UnitDefinitionBuilder for constructing a UnitDefinition with a fluent API.
    ///
    /// This method provides a builder pattern interface for creating and configuring
    /// a new UnitDefinition within this model. The builder allows chaining method calls
    /// to set various properties of the UnitDefinition before building it.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new unit definition
    /// * `name` - The name of the unit definition
    ///
    /// # Returns
    /// A UnitDefinitionBuilder instance that can be used to configure and create the UnitDefinition
    pub fn build_unit_definition(&self, id: &str, name: &str) -> UnitDefinitionBuilder<'a> {
        UnitDefinitionBuilder::new(self, id, name)
    }

    /// Returns a vector of all unit definitions in the model.
    ///
    /// # Returns
    /// A vector containing Rc references to all UnitDefinitions in the model
    pub fn list_of_unit_definitions(&self) -> Vec<Rc<UnitDefinition<'a>>> {
        self.list_of_unit_definitions.borrow().to_vec()
    }

    /// Retrieves a unit definition from the model by its identifier.
    ///
    /// # Arguments
    /// * `id` - The identifier of the unit definition to retrieve
    ///
    /// # Returns
    /// Some(`Rc<UnitDefinition>`) if found, None if not found
    pub fn get_unit_definition(&self, id: &str) -> Option<Rc<UnitDefinition<'a>>> {
        self.list_of_unit_definitions
            .borrow()
            .iter()
            .find(|unit_definition| (*unit_definition).id() == id)
            .map(Rc::clone)
    }

    /// Creates a new Reaction within this model.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new reaction
    ///
    /// # Returns
    /// A new Reaction instance wrapped in an Rc
    pub fn create_reaction(&self, id: &str) -> Rc<Reaction<'a>> {
        let reaction = Rc::new(Reaction::new(self, id));
        self.list_of_reactions
            .borrow_mut()
            .push(Rc::clone(&reaction));
        reaction
    }

    /// Creates a new ReactionBuilder for constructing a Reaction with a fluent API.
    ///
    /// This method provides a builder pattern interface for creating and configuring
    /// a new Reaction within this model. The builder allows chaining method calls
    /// to set various properties of the Reaction before building it.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new reaction
    ///
    /// # Returns
    /// A ReactionBuilder instance that can be used to configure and create the Reaction
    pub fn build_reaction(&self, id: &str) -> ReactionBuilder<'a> {
        ReactionBuilder::new(self, id)
    }

    /// Returns a vector of all reactions in the model.
    ///
    /// # Returns
    /// A vector containing Rc references to all Reactions in the model
    pub fn list_of_reactions(&self) -> Vec<Rc<Reaction<'a>>> {
        self.list_of_reactions.borrow().to_vec()
    }

    /// Retrieves a reaction from the model by its identifier.
    ///
    /// # Arguments
    /// * `id` - The identifier of the reaction to retrieve
    ///
    /// # Returns
    /// Some(`Rc<Reaction>`) if found, None if not found
    pub fn get_reaction(&self, id: &str) -> Option<Rc<Reaction<'a>>> {
        self.list_of_reactions
            .borrow()
            .iter()
            .find(|reaction| (*reaction).id() == id)
            .map(Rc::clone)
    }

    /// Creates a new Parameter within this model.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new parameter
    ///
    /// # Returns
    /// A new Parameter instance wrapped in an Rc
    pub fn create_parameter(&self, id: &str) -> Rc<Parameter<'a>> {
        let parameter = Rc::new(Parameter::new(self, id));
        self.list_of_parameters
            .borrow_mut()
            .push(Rc::clone(&parameter));
        parameter
    }

    /// Creates a new ParameterBuilder for constructing a Parameter with a fluent API.
    ///
    /// This method provides a builder pattern interface for creating and configuring
    /// a new Parameter within this model. The builder allows chaining method calls
    /// to set various properties of the Parameter before building it.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new parameter
    ///
    /// # Returns
    /// A ParameterBuilder instance that can be used to configure and create the Parameter
    pub fn build_parameter(&self, id: &str) -> ParameterBuilder<'a> {
        ParameterBuilder::new(self, id)
    }

    /// Returns a vector of all parameters in the model.
    ///
    /// # Returns
    /// A vector containing Rc references to all Parameters in the model
    pub fn list_of_parameters(&self) -> Vec<Rc<Parameter<'a>>> {
        self.list_of_parameters.borrow().to_vec()
    }

    /// Retrieves a parameter from the model by its identifier.
    ///
    /// # Arguments
    /// * `id` - The identifier of the parameter to retrieve
    ///
    /// # Returns
    /// Some(`Rc<Parameter>`) if found, None if not found
    pub fn get_parameter(&self, id: &str) -> Option<Rc<Parameter<'a>>> {
        self.list_of_parameters
            .borrow()
            .iter()
            .find(|parameter| (*parameter).id() == id)
            .map(Rc::clone)
    }

    /// Creates a new RateRule within this model.
    ///
    /// # Arguments
    /// * `variable` - The variable to apply the rate rule to
    /// * `formula` - The formula for the rate rule
    ///
    /// # Returns
    /// A new RateRule instance wrapped in an Rc
    pub fn create_rate_rule(&self, variable: impl IntoId<'a>, formula: &str) -> Rc<Rule<'a>> {
        let rate_rule = Rc::new(Rule::new_rate_rule(self, variable, formula));
        self.list_of_rate_rules
            .borrow_mut()
            .push(Rc::clone(&rate_rule));
        rate_rule
    }

    /// Creates a new RateRuleBuilder for constructing a RateRule with a fluent API.
    ///
    /// This method provides a builder pattern interface for creating and configuring
    /// a new RateRule within this model. The builder allows chaining method calls
    /// to set various properties of the RateRule before building it.
    ///
    /// # Arguments
    /// * `variable` - The variable to apply the rate rule to
    /// * `formula` - The formula for the rate rule
    ///
    /// # Returns
    /// A RateRuleBuilder instance that can be used to configure and create the RateRule
    pub fn build_rate_rule(&self, variable: impl IntoId<'a>, formula: &str) -> RateRuleBuilder<'a> {
        RateRuleBuilder::new(self, variable, formula)
    }

    /// Returns a vector of all rate rules in the model.
    ///
    /// # Returns
    /// A vector containing Rc references to all RateRules in the model
    pub fn list_of_rate_rules(&self) -> Vec<Rc<Rule<'a>>> {
        self.list_of_rate_rules.borrow().to_vec()
    }

    /// Retrieves a rate rule from the model by its identifier.
    ///
    /// # Arguments
    /// * `variable` - The variable to apply the rate rule to
    ///
    /// # Returns
    /// Some(`Rc<Rule>`) if found, None if not found
    pub fn get_rate_rule(&self, variable: &str) -> Option<Rc<Rule<'a>>> {
        self.list_of_rate_rules
            .borrow()
            .iter()
            .find(|rule| (*rule).variable() == variable)
            .map(Rc::clone)
    }

    /// Creates a new AssignmentRule within this model.
    ///
    /// # Arguments
    /// * `variable` - The variable to apply the assignment rule to
    /// * `formula` - The formula for the assignment rule
    ///
    /// # Returns
    /// A new AssignmentRule instance wrapped in an Rc
    pub fn create_assignment_rule(&self, variable: impl IntoId<'a>, formula: &str) -> Rc<Rule<'a>> {
        let assignment_rule = Rc::new(Rule::new_assignment_rule(self, variable, formula));
        self.list_of_assignment_rules
            .borrow_mut()
            .push(Rc::clone(&assignment_rule));
        assignment_rule
    }

    /// Creates a new AssignmentRuleBuilder for constructing a AssignmentRule with a fluent API.
    ///
    /// This method provides a builder pattern interface for creating and configuring
    /// a new AssignmentRule within this model. The builder allows chaining method calls
    /// to set various properties of the AssignmentRule before building it.
    ///
    /// # Arguments
    /// * `variable` - The variable to apply the assignment rule to
    /// * `formula` - The formula for the assignment rule
    ///
    /// # Returns
    /// A AssignmentRuleBuilder instance that can be used to configure and create the AssignmentRule
    pub fn build_assignment_rule(
        &self,
        variable: impl IntoId<'a>,
        formula: &str,
    ) -> AssignmentRuleBuilder<'a> {
        AssignmentRuleBuilder::new(self, variable, formula)
    }

    /// Returns a vector of all assignment rules in the model.
    ///
    /// # Returns
    /// A vector containing Rc references to all AssignmentRules in the model
    pub fn list_of_assignment_rules(&self) -> Vec<Rc<Rule<'a>>> {
        self.list_of_assignment_rules.borrow().to_vec()
    }

    /// Retrieves a assignment rule from the model by its identifier.
    ///
    /// # Arguments
    /// * `variable` - The variable to apply the assignment rule to
    ///
    /// # Returns
    /// Some(`Rc<Rule>`) if found, None if not found
    pub fn get_assignment_rule(&self, variable: &str) -> Option<Rc<Rule<'a>>> {
        self.list_of_assignment_rules
            .borrow()
            .iter()
            .find(|rule| (*rule).variable() == variable)
            .map(Rc::clone)
    }

    // Implement the set_annotation method for the Model type
    set_collection_annotation!(Model<'a>, "reactions", ListOfReactions);
    set_collection_annotation!(Model<'a>, "species", ListOfSpecies);
    set_collection_annotation!(Model<'a>, "compartments", ListOfCompartments);
    set_collection_annotation!(Model<'a>, "unit_definitions", ListOfUnitDefinitions);
    set_collection_annotation!(Model<'a>, "parameters", ListOfParameters);
    set_collection_annotation!(Model<'a>, "rate_rules", ListOfRules);

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::Model, sbmlcxx::SBase);
}

/// Implementation of the FromPtr trait for the Model type
///
/// This implementation allows the Model type to be created from a raw pointer to a libSBML Model.
/// It provides a consistent way to convert unsafe C++ pointers to safe Rust types.
///
/// The main usage is to parse a libSBML Model pointer into a Model instance from an SBMLReader.
///
/// # Type Parameters
/// * `T` - The raw pointer type from libSBML (e.g. sbmlcxx::Model)
///
/// # Returns
/// A new Model instance
impl<'a> FromPtr<sbmlcxx::Model> for Model<'a> {
    fn from_ptr(ptr: *mut sbmlcxx::Model) -> Self {
        let model = RefCell::new(pin_ptr!(ptr, sbmlcxx::Model));

        // Fetch all species
        let n_species = model.borrow().getNumSpecies().0;
        let list_of_species: Vec<_> = (0..n_species)
            .map(|i| {
                let species = model.borrow_mut().as_mut().getSpecies1(i.into());
                let species = Rc::new(Species::from_ptr(species));
                Rc::clone(&species)
            })
            .collect();

        // Fetch all compartments
        let n_compartments = model.borrow().getNumCompartments().0;
        let list_of_compartments: Vec<_> = (0..n_compartments)
            .map(|i| {
                let compartment = model.borrow_mut().as_mut().getCompartment1(i.into());
                let compartment = Rc::new(Compartment::from_ptr(compartment));
                Rc::clone(&compartment)
            })
            .collect();

        // Fetch all unit definitions
        let n_unit_definitions = model.borrow().getNumUnitDefinitions().0;
        let list_of_unit_definitions: Vec<_> = (0..n_unit_definitions)
            .map(|i| {
                let unit_definition = model.borrow_mut().as_mut().getUnitDefinition1(i.into());
                let unit_definition = Rc::new(UnitDefinition::from_ptr(unit_definition));
                Rc::clone(&unit_definition)
            })
            .collect();

        // Fetch all reactions
        let n_reactions = model.borrow().getNumReactions().0;
        let list_of_reactions: Vec<_> = (0..n_reactions)
            .map(|i| {
                let reaction = model.borrow_mut().as_mut().getReaction1(i.into());
                let reaction = Rc::new(Reaction::from_ptr(reaction));
                Rc::clone(&reaction)
            })
            .collect();

        // Fetch all parameters
        let n_parameters = model.borrow().getNumParameters().0;
        let list_of_parameters: Vec<_> = (0..n_parameters)
            .map(|i| {
                let parameter = model.borrow_mut().as_mut().getParameter1(i.into());
                let parameter = Rc::new(Parameter::from_ptr(parameter));
                Rc::clone(&parameter)
            })
            .collect();

        // Fetch all rate rules
        let n_rate_rules = model.borrow().getNumRules().0;
        let mut list_of_rate_rules: Vec<_> = Vec::new();
        let mut list_of_assignment_rules: Vec<_> = Vec::new();

        for i in 0..n_rate_rules {
            let mut model_mut = model.borrow_mut();
            let rule = model_mut.as_mut().getRule1(i.into());
            let rule = Rc::new(Rule::from_ptr(rule));
            match rule.rule_type() {
                Ok(RuleType::RateRule) => list_of_rate_rules.push(Rc::clone(&rule)),
                Ok(RuleType::AssignmentRule) => list_of_assignment_rules.push(Rc::clone(&rule)),
                Err(e) => println!("{}", e),
            }
        }

        Self {
            inner: model,
            list_of_species: RefCell::new(list_of_species),
            list_of_compartments: RefCell::new(list_of_compartments),
            list_of_unit_definitions: RefCell::new(list_of_unit_definitions),
            list_of_reactions: RefCell::new(list_of_reactions),
            list_of_parameters: RefCell::new(list_of_parameters),
            list_of_rate_rules: RefCell::new(list_of_rate_rules),
            list_of_assignment_rules: RefCell::new(list_of_assignment_rules),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_new() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model.set_id("test2");
        model.set_name("test2");

        assert_eq!(model.id(), "test2");
        assert_eq!(model.name(), "test2");
    }

    #[test]
    fn test_model_build_species() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let species = model.build_species("glucose").build();
        assert_eq!(species.id(), "glucose");
    }

    #[test]
    fn test_model_species() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let species = model.create_species("glucose");
        assert_eq!(species.id(), "glucose");
    }

    #[test]
    fn test_get_species() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.create_species("glucose");

        let extracted = model.get_species("glucose").expect("Species not found");
        assert_eq!(extracted.id(), "glucose");
    }

    #[test]
    fn test_get_species_not_found() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let extracted = model.get_species("glucose");
        assert!(extracted.is_none());
    }

    #[test]
    fn test_list_of_species() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.create_species("glucose");
        model.create_species("fructose");
        assert_eq!(model.list_of_species().len(), 2);
    }

    #[test]
    fn test_model_build_compartment() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let compartment = model.build_compartment("cytosol").build();
        assert_eq!(compartment.id(), "cytosol");
        assert_eq!(model.list_of_compartments().len(), 1);
    }

    #[test]
    fn test_model_compartments() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let compartment = model.create_compartment("cytosol");
        assert_eq!(compartment.id(), "cytosol");
    }

    #[test]
    fn test_get_compartment() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.create_compartment("cytosol");

        let extracted = model
            .get_compartment("cytosol")
            .expect("Compartment not found");
        assert_eq!(extracted.id(), "cytosol");
    }

    #[test]
    fn test_list_of_compartments() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.create_compartment("cytosol");
        model.create_compartment("nucleus");
        assert_eq!(model.list_of_compartments().len(), 2);
    }

    #[test]
    fn test_get_compartment_not_found() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let extracted = model.get_compartment("cytosol");
        assert!(extracted.is_none());
    }

    #[test]
    fn test_model_build_unit_definition() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let unit_definition = model.build_unit_definition("test", "test").build();
        assert_eq!(unit_definition.id(), "test");
    }

    #[test]
    fn test_model_unit_definitions() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let unit_definition = model.create_unit_definition("test", "test");
        assert_eq!(unit_definition.id(), "test");
    }

    #[test]
    fn test_get_unit_definition() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.create_unit_definition("test", "test");

        let extracted = model
            .get_unit_definition("test")
            .expect("UnitDefinition not found");
        assert_eq!(extracted.id(), "test");
    }

    #[test]
    fn test_get_unit_definition_not_found() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let extracted = model.get_unit_definition("test");
        assert!(extracted.is_none());
    }

    #[test]
    fn test_model_build_reaction() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let reaction = model.build_reaction("test").build();
        assert_eq!(reaction.id(), "test");
    }

    #[test]
    fn test_get_reaction() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.create_reaction("test");

        let extracted = model.get_reaction("test").expect("Reaction not found");
        assert_eq!(extracted.id(), "test");
    }

    #[test]
    fn test_get_reaction_not_found() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let extracted = model.get_reaction("test");
        assert!(extracted.is_none());
    }

    #[test]
    fn test_list_of_reactions() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.create_reaction("test");
        model.create_reaction("test2");
        assert_eq!(model.list_of_reactions().len(), 2);
    }

    #[test]
    fn test_model_build_parameter() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let parameter = model.build_parameter("test").build();
        assert_eq!(parameter.id(), "test");
    }

    #[test]
    fn test_model_parameters() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let parameter = model.create_parameter("test");
        assert_eq!(parameter.id(), "test");
    }

    #[test]
    fn test_get_parameter() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.create_parameter("test");

        let extracted = model.get_parameter("test").expect("Parameter not found");
        assert_eq!(extracted.id(), "test");
    }

    #[test]
    fn test_get_parameter_not_found() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let extracted = model.get_parameter("test");
        assert!(extracted.is_none());
    }

    #[test]
    fn test_model_build_rate_rule() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let rate_rule = model.build_rate_rule("test", "test").build();
        assert_eq!(rate_rule.variable(), "test");
        assert_eq!(rate_rule.formula(), "test");
    }

    #[test]
    fn test_model_rate_rules() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let rule = model.build_rate_rule("test", "test").build();
        assert_eq!(rule.variable(), "test");
        assert_eq!(rule.formula(), "test");
    }

    #[test]
    fn test_list_of_rules() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.build_rate_rule("test", "test").build();
        model.build_rate_rule("test2", "test2").build();
        assert_eq!(model.list_of_rate_rules().len(), 2);
    }

    #[test]
    fn test_get_rate_rule() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.build_rate_rule("test", "test").build();
        let rule = model.get_rate_rule("test").expect("RateRule not found");
        assert_eq!(rule.variable(), "test");
        assert_eq!(rule.formula(), "test");
    }

    #[test]
    fn test_get_rate_rule_not_found() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let rule = model.get_rate_rule("test");
        assert!(rule.is_none());
    }

    #[test]
    fn test_model_build_assignment_rule() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let assignment_rule = model.build_assignment_rule("test", "test").build();
        assert_eq!(assignment_rule.variable(), "test");
        assert_eq!(assignment_rule.formula(), "test");
    }

    #[test]
    fn test_model_assignment_rules() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let assignment_rule = model.build_assignment_rule("test", "test").build();
        assert_eq!(assignment_rule.variable(), "test");
        assert_eq!(assignment_rule.formula(), "test");
    }

    #[test]
    fn test_list_of_assignment_rules() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.build_assignment_rule("test", "test").build();
        model.build_assignment_rule("test2", "test2").build();
        assert_eq!(model.list_of_assignment_rules().len(), 2);
    }

    #[test]
    fn test_get_assignment_rule() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.build_assignment_rule("test", "test").build();
        let rule = model
            .get_assignment_rule("test")
            .expect("AssignmentRule not found");
        assert_eq!(rule.variable(), "test");
        assert_eq!(rule.formula(), "test");
    }

    #[test]
    fn test_get_assignment_rule_not_found() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let rule = model.get_assignment_rule("test");
        assert!(rule.is_none());
    }

    #[test]
    fn test_set_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.set_annotation("<test>test</test>").unwrap();
        assert_eq!(
            model.get_annotation().replace("\n", "").replace(" ", ""),
            "<annotation><test>test</test></annotation>"
        );
    }

    #[test]
    fn test_set_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model
            .set_annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .unwrap();

        let annotation: TestAnnotation = model.get_annotation_serde().unwrap();
        assert_eq!(annotation.test, "test");
    }

    #[test]
    #[should_panic]
    fn test_set_annotation_serde_invalid() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.set_annotation_serde(&"invalid").unwrap();
    }

    // Reactions Annotation Tests
    #[test]
    fn test_set_reactions_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model.create_reaction("r1");
        model.create_reaction("r2");

        model
            .set_reactions_annotation("<test>reactions</test>")
            .unwrap();
        assert_eq!(
            model
                .get_reactions_annotation()
                .replace("\n", "")
                .replace(" ", ""),
            "<annotation><test>reactions</test></annotation>"
        );
    }

    #[test]
    fn test_set_reactions_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model
            .set_reactions_annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .unwrap();

        let annotation: TestAnnotation = model.get_reactions_annotation_serde().unwrap();
        assert_eq!(annotation.test, "test");
    }

    #[test]
    #[should_panic]
    fn test_set_reactions_annotation_serde_invalid() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.set_reactions_annotation_serde(&"invalid").unwrap();
    }

    // Species Annotation Tests
    #[test]
    fn test_set_species_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model.create_species("s1");
        model.create_species("s2");

        model
            .set_species_annotation("<test>species</test>")
            .unwrap();
        assert_eq!(
            model
                .get_species_annotation()
                .replace("\n", "")
                .replace(" ", ""),
            "<annotation><test>species</test></annotation>"
        );
    }

    #[test]
    fn test_set_species_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model
            .set_species_annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .unwrap();

        let annotation: TestAnnotation = model.get_species_annotation_serde().unwrap();
        assert_eq!(annotation.test, "test");
    }

    #[test]
    #[should_panic]
    fn test_set_species_annotation_serde_invalid() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.set_species_annotation_serde(&"invalid").unwrap();
    }

    // Compartments Annotation Tests
    #[test]
    fn test_set_compartments_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model.create_compartment("c1");
        model.create_compartment("c2");

        model
            .set_compartments_annotation("<test>compartments</test>")
            .unwrap();
        assert_eq!(
            model
                .get_compartments_annotation()
                .replace("\n", "")
                .replace(" ", ""),
            "<annotation><test>compartments</test></annotation>"
        );
    }

    #[test]
    fn test_set_compartments_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model
            .set_compartments_annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .unwrap();

        let annotation: TestAnnotation = model.get_compartments_annotation_serde().unwrap();
        assert_eq!(annotation.test, "test");
    }

    #[test]
    #[should_panic]
    fn test_set_compartments_annotation_serde_invalid() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.set_compartments_annotation_serde(&"invalid").unwrap();
    }

    // Unit Definitions Annotation Tests
    #[test]
    fn test_set_unit_definitions_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model.create_unit_definition("u1", "unit1");
        model.create_unit_definition("u2", "unit2");

        model
            .set_unit_definitions_annotation("<test>units</test>")
            .unwrap();
        assert_eq!(
            model
                .get_unit_definitions_annotation()
                .replace("\n", "")
                .replace(" ", ""),
            "<annotation><test>units</test></annotation>"
        );
    }

    #[test]
    fn test_set_unit_definitions_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model
            .set_unit_definitions_annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .unwrap();

        let annotation: TestAnnotation = model.get_unit_definitions_annotation_serde().unwrap();
        assert_eq!(annotation.test, "test");
    }

    #[test]
    #[should_panic]
    fn test_set_unit_definitions_annotation_serde_invalid() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model
            .set_unit_definitions_annotation_serde(&"invalid")
            .unwrap();
    }

    // Parameters Annotation Tests
    #[test]
    fn test_set_parameters_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model.create_parameter("p1");
        model.create_parameter("p2");

        model
            .set_parameters_annotation("<test>parameters</test>")
            .unwrap();
        assert_eq!(
            model
                .get_parameters_annotation()
                .replace("\n", "")
                .replace(" ", ""),
            "<annotation><test>parameters</test></annotation>"
        );
    }

    #[test]
    fn test_set_parameters_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model
            .set_parameters_annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .unwrap();

        let annotation: TestAnnotation = model.get_parameters_annotation_serde().unwrap();
        assert_eq!(annotation.test, "test");
    }

    #[test]
    #[should_panic]
    fn test_set_parameters_annotation_serde_invalid() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.set_parameters_annotation_serde(&"invalid").unwrap();
    }

    // Rate Rules Annotation Tests
    #[test]
    fn test_set_rate_rules_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model.create_rate_rule("r1", "x + y");
        model.create_rate_rule("r2", "a + b");

        model
            .set_rate_rules_annotation("<test>rules</test>")
            .unwrap();
        assert_eq!(
            model
                .get_rate_rules_annotation()
                .replace("\n", "")
                .replace(" ", ""),
            "<annotation><test>rules</test></annotation>"
        );
    }

    #[test]
    fn test_set_rate_rules_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");

        model
            .set_rate_rules_annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .unwrap();

        let annotation: TestAnnotation = model.get_rate_rules_annotation_serde().unwrap();
        assert_eq!(annotation.test, "test");
    }

    #[test]
    #[should_panic]
    fn test_set_rate_rules_annotation_serde_invalid() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        model.set_rate_rules_annotation_serde(&"invalid").unwrap();
    }
}
