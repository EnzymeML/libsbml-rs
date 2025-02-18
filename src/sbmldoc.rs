//! SBML Document handling
//!
//! This module provides a safe Rust wrapper around libSBML's SBMLDocument class.
//! The Systems Biology Markup Language (SBML) is an XML-based format for representing
//! computational models in systems biology. An SBMLDocument is the root container
//! for all SBML content.

use std::cell::RefCell;

use autocxx::{c_uint, WithinUniquePtr};
use cxx::UniquePtr;

use crate::{model::Model, sbmlcxx};

/// A wrapper around libSBML's SBMLDocument class that provides a safe Rust interface.
///
/// The SBMLDocument is the top-level container for an SBML model and associated data.
/// It maintains the SBML level and version, and contains a single optional Model.
pub struct SBMLDocument<'a> {
    /// The underlying libSBML document, wrapped in RefCell to allow interior mutability
    document: RefCell<UniquePtr<sbmlcxx::SBMLDocument>>,
    /// The optional Model contained in this document
    model: Option<Model<'a>>,
}

impl<'a> SBMLDocument<'a> {
    /// Creates a new SBMLDocument with the specified SBML level and version.
    ///
    /// # Arguments
    /// * `level` - The SBML Level of the document (e.g. 3)
    /// * `version` - The Version within the SBML Level (e.g. 2)
    ///
    /// # Returns
    /// A new SBMLDocument instance
    pub fn new(level: u32, version: u32) -> Self {
        let document = sbmlcxx::SBMLDocument::new(c_uint::from(level), c_uint::from(version))
            .within_unique_ptr();
        Self {
            document: RefCell::new(document),
            model: None,
        }
    }

    /// Returns a reference to the underlying libSBML document.
    ///
    /// This is primarily for internal use by other parts of the API.
    pub(crate) fn inner(&self) -> &RefCell<UniquePtr<sbmlcxx::SBMLDocument>> {
        &self.document
    }

    /// Creates a new Model within this document with the given ID.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new Model
    ///
    /// # Returns
    /// A reference to the newly created Model
    pub fn create_model(&'a mut self, id: &str) -> &'a Model<'a> {
        let model = Model::new(self, id);
        self.model = Some(model);
        self.model.as_ref().unwrap()
    }

    /// Returns a reference to the Model if one exists.
    pub fn model(&self) -> Option<&Model<'a>> {
        self.model.as_ref()
    }

    /// Returns a mutable reference to the Model if one exists.
    pub fn model_mut(&'a mut self) -> Option<&'a mut Model<'a>> {
        self.model.as_mut()
    }
}
