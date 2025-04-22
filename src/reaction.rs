//! This module provides a safe Rust interface to the libSBML Reaction class.
//!
//! The Reaction class represents a chemical or biological reaction in an SBML model.
//! It can represent reactions, such as enzyme-catalyzed reactions, or other biochemical
//! processes. Each reaction can have properties like kinetic laws, reactants, products,
//! and stoichiometry.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Reaction class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc};

use cxx::let_cxx_string;

use crate::{
    clone, inner, into_id,
    model::Model,
    modref::{ModifierSpeciesReference, ModifierSpeciesReferenceBuilder},
    pin_ptr,
    prelude::IntoId,
    sbmlcxx::{self},
    sbo_term,
    speciesref::{SpeciesReference, SpeciesReferenceBuilder, SpeciesReferenceType},
    traits::fromptr::FromPtr,
    upcast_annotation,
};

/// A safe wrapper around the libSBML Reaction class.
///
/// This struct maintains a reference to the underlying C++ Reaction object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
/// It also maintains vectors of reactants and products associated with the reaction.
pub struct Reaction<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::Reaction>>,
    reactants: RefCell<Vec<Rc<SpeciesReference<'a>>>>,
    products: RefCell<Vec<Rc<SpeciesReference<'a>>>>,
    modifiers: RefCell<Vec<Rc<ModifierSpeciesReference<'a>>>>,
}

// Set the inner trait for the Reaction struct
inner!(sbmlcxx::Reaction, Reaction<'a>);

// Set the annotation trait for the Reaction struct
upcast_annotation!(Reaction<'a>, sbmlcxx::Reaction, sbmlcxx::SBase);

// Set the into_id trait for the Reaction struct
into_id!(&Rc<Reaction<'_>>, id);

// Implement the Clone trait for the Reaction struct
clone!(
    Reaction<'a>,
    sbmlcxx::Reaction,
    reactants,
    products,
    modifiers
);

impl<'a> Reaction<'a> {
    /// Creates a new Reaction instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this reaction
    /// * `id` - The identifier for this reaction
    ///
    /// # Returns
    /// A new Reaction instance
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let reaction_ptr = model.inner().borrow_mut().as_mut().createReaction();
        let mut reaction = pin_ptr!(reaction_ptr, sbmlcxx::Reaction);

        // Set the id of the reaction
        let_cxx_string!(id = id);
        reaction.as_mut().setId(&id);

        Self {
            inner: RefCell::new(reaction),
            reactants: RefCell::new(Vec::new()),
            products: RefCell::new(Vec::new()),
            modifiers: RefCell::new(Vec::new()),
        }
    }

    /// Returns a reference to the inner RefCell containing the Reaction pointer.
    ///
    /// This is primarily used internally by other parts of the library.
    pub(crate) fn inner(&self) -> &RefCell<Pin<&'a mut sbmlcxx::Reaction>> {
        &self.inner
    }

    /// Returns the id of the reaction.
    ///
    /// # Returns
    /// The id of the reaction as a String
    pub fn id(&self) -> String {
        self.inner.borrow().getId().to_str().unwrap().to_string()
    }

    /// Sets the id of the reaction.
    ///
    /// # Arguments
    /// * `id` - The id to set
    pub fn set_id(&self, id: &str) {
        let_cxx_string!(id = id);
        self.inner.borrow_mut().as_mut().setId(&id);
    }

    /// Returns the name of the reaction.
    ///
    /// # Returns
    /// The name of the reaction as a String
    pub fn name(&self) -> String {
        self.inner.borrow().getName().to_str().unwrap().to_string()
    }

    /// Sets the name of the reaction.
    ///
    /// # Arguments
    /// * `name` - The name to set
    pub fn set_name(&self, name: &str) {
        let_cxx_string!(name = name);
        self.inner.borrow_mut().as_mut().setName(&name);
    }

    /// Creates a new product species reference for this reaction.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the product
    /// * `stoichiometry` - The stoichiometric coefficient for the product
    ///
    /// # Returns
    /// A reference-counted pointer to the new SpeciesReference
    pub fn create_product(
        &self,
        sid: impl IntoId<'a>,
        stoichiometry: f64,
    ) -> Rc<SpeciesReference<'a>> {
        let product = Rc::new(SpeciesReference::new(
            self,
            sid,
            SpeciesReferenceType::Product,
        ));
        product.set_stoichiometry(stoichiometry);
        self.products.borrow_mut().push(Rc::clone(&product));
        product
    }

    /// Creates a builder for a new product species reference.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the product
    ///
    /// # Returns
    /// A SpeciesReferenceBuilder for configuring and creating the product
    pub fn build_product(&self, sid: impl IntoId<'a>) -> SpeciesReferenceBuilder<'a> {
        SpeciesReferenceBuilder::new(&self, sid, SpeciesReferenceType::Product)
    }

    /// Returns a reference to the products of this reaction.
    ///
    /// # Returns
    /// A reference to the RefCell containing the vector of products
    pub fn products(&self) -> &RefCell<Vec<Rc<SpeciesReference<'a>>>> {
        &self.products
    }

    /// Returns a reference to the product with the given species id.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the product
    ///
    /// # Returns
    /// An Option containing a reference-counted pointer to the SpeciesReference if found
    pub fn get_product(&self, sid: &str) -> Option<Rc<SpeciesReference<'a>>> {
        self.products
            .borrow()
            .iter()
            .find(|product| (*product).species() == sid)
            .map(|product| Rc::clone(product))
    }

    /// Creates a new reactant species reference for this reaction.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the reactant
    /// * `stoichiometry` - The stoichiometric coefficient for the reactant
    ///
    /// # Returns
    /// A reference-counted pointer to the new SpeciesReference
    pub fn create_reactant(
        &self,
        sid: impl IntoId<'a>,
        stoichiometry: f64,
    ) -> Rc<SpeciesReference<'a>> {
        let reactant = Rc::new(SpeciesReference::new(
            self,
            sid,
            SpeciesReferenceType::Reactant,
        ));
        reactant.set_stoichiometry(stoichiometry);
        self.reactants.borrow_mut().push(Rc::clone(&reactant));
        reactant
    }

    /// Creates a builder for a new reactant species reference.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the reactant
    ///
    /// # Returns
    /// A SpeciesReferenceBuilder for configuring and creating the reactant
    pub fn build_reactant(&self, sid: impl IntoId<'a>) -> SpeciesReferenceBuilder<'a> {
        SpeciesReferenceBuilder::new(&self, sid, SpeciesReferenceType::Reactant)
    }

    /// Returns a reference to the reactants of this reaction.
    ///
    /// # Returns
    /// A reference to the RefCell containing the vector of reactants
    pub fn reactants(&self) -> &RefCell<Vec<Rc<SpeciesReference<'a>>>> {
        &self.reactants
    }

    /// Returns a reference to the reactant with the given species id.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the reactant
    ///
    /// # Returns
    /// An Option containing a reference-counted pointer to the SpeciesReference if found
    pub fn get_reactant(&self, sid: &str) -> Option<Rc<SpeciesReference<'a>>> {
        self.reactants
            .borrow()
            .iter()
            .find(|reactant| (*reactant).species() == sid)
            .map(|reactant| Rc::clone(reactant))
    }

    /// Creates a new modifier species reference for this reaction.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the modifier
    ///
    /// # Returns
    /// A reference-counted pointer to the new ModifierSpeciesReference
    pub fn create_modifier(&self, sid: &str) -> Rc<ModifierSpeciesReference<'a>> {
        let modifier = Rc::new(ModifierSpeciesReference::new(self, sid));
        self.modifiers.borrow_mut().push(Rc::clone(&modifier));
        modifier
    }

    /// Creates a builder for a new modifier species reference.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the modifier
    ///
    /// # Returns
    /// A ModifierSpeciesReferenceBuilder for configuring and creating the modifier
    pub fn build_modifier(&self, sid: impl IntoId<'a>) -> ModifierSpeciesReferenceBuilder<'a> {
        ModifierSpeciesReferenceBuilder::new(&self, sid)
    }
    /// Returns a reference to the modifiers of this reaction.
    ///
    /// # Returns
    /// A reference to the RefCell containing the vector of modifiers
    pub fn modifiers(&self) -> &RefCell<Vec<Rc<ModifierSpeciesReference<'a>>>> {
        &self.modifiers
    }

    /// Returns a reference to the modifier with the given species id.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the modifier
    ///
    /// # Returns
    /// An Option containing a reference-counted pointer to the ModifierSpeciesReference if found
    pub fn get_modifier(&self, sid: &str) -> Option<Rc<ModifierSpeciesReference<'a>>> {
        self.modifiers
            .borrow()
            .iter()
            .find(|modifier| (*modifier).species() == sid)
            .map(|modifier| Rc::clone(modifier))
    }

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::Reaction, sbmlcxx::SBase);
}

impl FromPtr<sbmlcxx::Reaction> for Reaction<'_> {
    /// Creates a new Reaction instance from a unique pointer to a libSBML Reaction.
    ///
    /// This method is primarily used internally by the Model class to create
    /// Reaction instances from libSBML Reaction pointers.
    ///
    /// # Arguments
    /// * `ptr` - A unique pointer to a libSBML Reaction
    ///
    /// # Returns
    /// A new Reaction instance
    fn from_ptr(ptr: *mut sbmlcxx::Reaction) -> Self {
        let reaction = pin_ptr!(ptr, sbmlcxx::Reaction);
        Self {
            inner: RefCell::new(reaction),
            reactants: RefCell::new(Vec::new()),
            products: RefCell::new(Vec::new()),
            modifiers: RefCell::new(Vec::new()),
        }
    }
}
/// A builder for creating Reaction instances with a fluent interface.
pub struct ReactionBuilder<'a> {
    reaction: Rc<Reaction<'a>>,
}

impl<'a> ReactionBuilder<'a> {
    /// Creates a new ReactionBuilder instance.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain the reaction
    /// * `id` - The identifier for the reaction
    ///
    /// # Returns
    /// A new ReactionBuilder instance
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let reaction = model.create_reaction(id);
        Self { reaction }
    }

    /// Sets the name of the reaction.
    ///
    /// # Arguments
    /// * `name` - The name to set
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn name(self, name: &str) -> Self {
        self.reaction.set_name(name);
        self
    }

    /// Adds a product to the reaction being built.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the product
    /// * `stoichiometry` - The stoichiometric coefficient for the product
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn product(self, sid: impl IntoId<'a>, stoichiometry: f64) -> Self {
        self.reaction.create_product(sid.into_id(), stoichiometry);
        self
    }

    /// Adds a reactant to the reaction being built.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the reactant
    /// * `stoichiometry` - The stoichiometric coefficient for the reactant
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn reactant(self, sid: impl IntoId<'a>, stoichiometry: f64) -> Self {
        self.reaction.create_reactant(sid.into_id(), stoichiometry);
        self
    }

    /// Adds a modifier to the reaction being built.
    ///
    /// # Arguments
    /// * `sid` - The species identifier for the modifier
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn modifier(self, sid: impl IntoId<'a>) -> Self {
        self.reaction.create_modifier(sid.into_id());
        self
    }

    pub fn build(self) -> Rc<Reaction<'a>> {
        self.reaction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_reaction_new() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = Reaction::new(&model, "test");

        reaction.set_id("test2");
        reaction.set_name("test2");

        assert_eq!(reaction.id(), "test2");
        assert_eq!(reaction.name(), "test2");
    }

    #[test]
    fn test_reaction_builder() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        assert_eq!(reaction.id(), "test");
    }

    #[test]
    fn test_reaction_builder_product() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        let product = reaction.build_product("test").stoichiometry(1.0).build();
        assert_eq!(product.species(), "test");
    }

    #[test]
    fn test_reaction_builder_reactant() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        let reactant = reaction.build_reactant("test").stoichiometry(1.0).build();

        assert_eq!(reactant.species(), "test");
        assert_eq!(reactant.stoichiometry(), 1.0);
    }

    #[test]
    fn test_reaction_builder_modifier() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        let modifier = reaction.build_modifier("test").build();

        assert_eq!(modifier.species(), "test");
    }

    #[test]
    fn test_reaction_builder_build() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test")
            .name("test")
            .product("test", 1.0)
            .reactant("test", 1.0)
            .modifier("test")
            .build();

        assert_eq!(reaction.name(), "test");

        let products = reaction.products();
        let reactants = reaction.reactants();
        let modifiers = reaction.modifiers();
        assert_eq!(products.borrow().len(), 1);
        assert_eq!(reactants.borrow().len(), 1);
        assert_eq!(modifiers.borrow().len(), 1);
        assert_eq!(reaction.id(), "test");

        let product = reaction.get_product("test").unwrap();
        let reactant = reaction.get_reactant("test").unwrap();
        let modifier = reaction.get_modifier("test").unwrap();

        assert_eq!(product.species(), "test");
        assert_eq!(product.stoichiometry(), 1.0);
        assert_eq!(reactant.species(), "test");
        assert_eq!(reactant.stoichiometry(), 1.0);
        assert_eq!(modifier.species(), "test");
    }

    #[test]
    fn test_reaction_builder_get_product() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        reaction.create_product("test", 1.0);

        let product = reaction.get_product("test").unwrap();
        assert_eq!(product.species(), "test");
    }

    #[test]
    fn test_reaction_builder_get_product_not_found() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        let product = reaction.get_product("test");
        assert!(product.is_none());
    }

    #[test]
    fn test_reaction_builder_get_reactant() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        reaction.create_reactant("test", 1.0);

        let reactant = reaction.get_reactant("test").unwrap();
        assert_eq!(reactant.species(), "test");
    }

    #[test]
    fn test_reaction_builder_get_reactant_not_found() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        let reactant = reaction.get_reactant("test");
        assert!(reactant.is_none());
    }

    #[test]
    fn test_reaction_builder_get_modifier() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        reaction.create_modifier("test");

        let modifier = reaction.get_modifier("test").unwrap();
        assert_eq!(modifier.species(), "test");
    }

    #[test]
    fn test_reaction_builder_get_modifier_not_found() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        let modifier = reaction.get_modifier("test");
        assert!(modifier.is_none());
    }

    #[test]
    fn test_reaction_builder_get_products() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();

        reaction.create_product("test", 1.0);
        reaction.create_product("test2", 2.0);
        let products = reaction.products();
        assert_eq!(products.borrow().len(), 2);
    }

    #[test]
    fn test_reaction_builder_get_reactants() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        reaction.create_reactant("test", 1.0);
        reaction.create_reactant("test2", 2.0);

        let reactants = reaction.reactants();
        assert_eq!(reactants.borrow().len(), 2);
    }

    #[test]
    fn test_reaction_builder_get_modifiers() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        reaction.create_modifier("test");

        let modifiers = reaction.modifiers();
        assert_eq!(modifiers.borrow().len(), 1);
    }

    #[test]
    fn test_annotation() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        reaction
            .set_annotation("<test>test</test>")
            .expect("Failed to set annotation");

        assert_eq!(
            reaction.get_annotation().replace("\n", "").replace(" ", ""),
            "<annotation><test>test</test></annotation>"
        );
    }

    #[test]
    fn test_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }
        let annotation = TestAnnotation {
            test: "test".to_string(),
        };
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let reaction = ReactionBuilder::new(&model, "test").build();
        reaction
            .set_annotation_serde(&annotation)
            .expect("Failed to set annotation");

        let annotation = reaction.get_annotation_serde::<TestAnnotation>().unwrap();
        assert_eq!(annotation.test, "test");
    }
}
