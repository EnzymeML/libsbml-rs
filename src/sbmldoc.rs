//! SBML Document handling
//!
//! This module provides a safe Rust wrapper around libSBML's SBMLDocument class.
//! The Systems Biology Markup Language (SBML) is an XML-based format for representing
//! computational models in systems biology. An SBMLDocument is the root container
//! for all SBML content.

use std::{cell::RefCell, rc::Rc};

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
    model: RefCell<Option<Rc<Model<'a>>>>,
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
            model: RefCell::new(None),
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
    pub fn create_model(&'a self, id: &str) -> Rc<Model<'a>> {
        let model = Rc::new(Model::new(self, id));
        self.model.borrow_mut().replace(Rc::clone(&model));
        model
    }

    /// Returns a reference to the Model if one exists.
    pub fn model(&self) -> Option<Rc<Model<'a>>> {
        self.model.borrow().as_ref().map(|model| Rc::clone(model))
    }

    /// Converts the SBML document to an XML string representation.
    ///
    /// This function uses the SBMLWriter to serialize the current state of the
    /// SBML document into an XML string. If the document is not available,
    /// an empty string is returned.
    ///
    /// # Returns
    /// A String containing the XML representation of the SBML document, or
    /// an empty String if the document is not available.
    pub fn to_xml_string(&self) -> String {
        let mut writer = sbmlcxx::SBMLWriter::new().within_unique_ptr();

        if let Some(doc) = self.document.borrow_mut().as_mut() {
            let raw_ptr: *mut sbmlcxx::SBMLDocument = unsafe { doc.get_unchecked_mut() as *mut _ };
            let string_ptr = unsafe { writer.pin_mut().writeSBMLToString(raw_ptr) };
            let string = unsafe { std::ffi::CStr::from_ptr(string_ptr) };
            string.to_string_lossy().into_owned()
        } else {
            String::new()
        }
    }
}
