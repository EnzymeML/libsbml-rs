//! SBML Document handling
//!
//! This module provides a safe Rust wrapper around libSBML's SBMLDocument class.
//! The Systems Biology Markup Language (SBML) is an XML-based format for representing
//! computational models in systems biology. An SBMLDocument is the root container
//! for all SBML content.

use std::{cell::RefCell, rc::Rc};

use autocxx::{c_uint, WithinUniquePtr};
use cxx::UniquePtr;

use crate::{cast::upcast, model::Model, sbmlcxx, traits::fromptr::FromPtr};

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

    /// Creates a new SBMLDocument from a unique pointer to a libSBML document.
    ///
    /// This is mainly used internally to construct SBMLDocument instances from
    /// XML strings and files - the SBMLReader uses this method to return a
    /// SBMLDocument instance from an XML source.
    ///
    /// # Arguments
    /// * `ptr` - A unique pointer to a libSBML document
    ///
    /// # Returns
    /// A new SBMLDocument instance
    pub(crate) fn from_unique_ptr(ptr: UniquePtr<sbmlcxx::SBMLDocument>) -> Self {
        // Wrap the pointer in a RefCell
        let document = RefCell::new(ptr);

        // Grab the model from the document
        let model = document
            .borrow_mut()
            .as_mut()
            .map(|model| Rc::new(Model::from_ptr(model.getModel1())));

        Self {
            document,
            model: RefCell::new(model),
        }
    }

    /// Returns a reference to the underlying libSBML document.
    ///
    /// This is primarily for internal use by other parts of the API.
    pub(crate) fn inner(&self) -> &RefCell<UniquePtr<sbmlcxx::SBMLDocument>> {
        &self.document
    }

    /// Returns the SBML level of the document.
    pub fn level(&self) -> u32 {
        let base = unsafe {
            upcast::<sbmlcxx::SBMLDocument, sbmlcxx::SBase>(self.document.borrow_mut().as_mut_ptr())
        };

        base.getLevel().0
    }

    /// Returns the SBML version of the document.
    pub fn version(&self) -> u32 {
        let base = unsafe {
            upcast::<sbmlcxx::SBMLDocument, sbmlcxx::SBase>(self.document.borrow_mut().as_mut_ptr())
        };

        base.getVersion().0
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
        self.model.borrow().as_ref().map(Rc::clone)
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

impl<'a> Default for SBMLDocument<'a> {
    fn default() -> Self {
        Self::new(3, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbmldoc_new() {
        let doc = SBMLDocument::new(3, 2);
        assert_eq!(doc.level(), 3);
        assert_eq!(doc.version(), 2);
    }

    #[test]
    fn test_sbmldoc_default() {
        let doc = SBMLDocument::default();
        assert_eq!(doc.level(), 3);
        assert_eq!(doc.version(), 2);
    }

    #[test]
    fn test_sbmldoc_create_model() {
        let doc = SBMLDocument::new(3, 2);
        doc.create_model("test");

        let model = doc.model().expect("Model not found");
        assert_eq!(model.id(), "test");
    }

    #[test]
    fn test_sbmldoc_to_xml_string() {
        let doc = SBMLDocument::new(3, 2);
        doc.create_model("test");

        let xml_string = doc.to_xml_string();
        assert!(!xml_string.is_empty());
    }
}
