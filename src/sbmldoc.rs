//! SBML Document handling
//!
//! This module provides a safe Rust wrapper around libSBML's SBMLDocument class.
//! The Systems Biology Markup Language (SBML) is an XML-based format for representing
//! computational models in systems biology. An SBMLDocument is the root container
//! for all SBML content.

use std::{cell::RefCell, rc::Rc};

use autocxx::WithinUniquePtr;
use cxx::{let_cxx_string, UniquePtr};

use crate::{
    cast::upcast,
    model::Model,
    namespaces::SBMLNamespaces,
    packages::{Package, PackageSpec},
    pin_const_ptr,
    prelude::SBMLErrorLog,
    sbmlcxx,
    traits::fromptr::FromPtr,
};

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
    pub fn new(level: u32, version: u32, packages: impl Into<Option<Vec<PackageSpec>>>) -> Self {
        let namespaces = SBMLNamespaces::new(level, version);

        // Add packages if provided
        if let Some(packages) = packages.into() {
            for package in packages {
                namespaces.add_package(package);
            }
        }

        let mut document =
            unsafe { sbmlcxx::SBMLDocument::new1(namespaces.inner().borrow_mut().as_mut_ptr()) }
                .within_unique_ptr();

        // Enable FBC
        if let Some(doc) = document.as_mut() {
            let_cxx_string!(fbc = "fbc");
            doc.setPackageRequired(&fbc, true);
        }

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
    pub(crate) fn from_unique_ptr(ptr: UniquePtr<sbmlcxx::SBMLDocument>) -> SBMLDocument<'static> {
        // Wrap the pointer in a RefCell
        let document = RefCell::new(ptr);

        // Grab the model from the document
        let model = document
            .borrow_mut()
            .as_mut()
            .map(|model| Rc::new(Model::from_ptr(model.getModel1())));

        SBMLDocument {
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

    /// Returns the number of plugins in the document.
    pub fn plugins(&self) -> Vec<String> {
        let base = unsafe {
            upcast::<sbmlcxx::SBMLDocument, sbmlcxx::SBase>(self.document.borrow_mut().as_mut_ptr())
        };

        // Get the number of plugins
        let n_plugins = base.getNumPlugins().0;

        // Get the plugin names
        let mut plugins = Vec::new();
        for i in 0..n_plugins {
            let plugin_ptr = base.getPlugin3(i.into());
            let plugin = pin_const_ptr!(plugin_ptr, sbmlcxx::SBasePlugin);
            plugins.push(plugin.getPackageName().to_string());
        }

        plugins
    }

    /// Creates a new Model within this document with the given ID.
    ///
    /// # Arguments
    /// * `id` - The identifier for the new Model
    ///
    /// # Returns
    /// A reference to the newly created Model
    pub fn create_model(&self, id: &str) -> Rc<Model<'a>> {
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

    /// Checks the consistency of the SBML document.
    ///
    /// This function performs a consistency check on the SBML document and returns
    /// a Result containing an error log if the document is not consistent. The [`SBMLErrorLog`]
    /// struct contains the validation status of the document and a list of all errors encountered
    /// during validation.
    ///
    /// Users can simply check the `valid` field of the returned [`SBMLErrorLog`] to determine
    /// if the document is consistent. If not, they can iterate over the `errors` vector to
    ///
    /// # Returns
    /// A [`SBMLErrorLog`] containing the validation status and errors of the document.
    pub fn check_consistency(&self) -> SBMLErrorLog {
        self.inner()
            .borrow_mut()
            .as_mut()
            .unwrap()
            .checkConsistency();

        SBMLErrorLog::new(self)
    }
}

impl<'a> std::fmt::Debug for SBMLDocument<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("SBMLDocument");
        ds.field("level", &self.level());
        ds.field("version", &self.version());
        ds.field("model", &self.model());
        ds.finish()
    }
}

impl<'a> Default for SBMLDocument<'a> {
    /// Creates a new SBMLDocument with the default SBML level and version, and FBC package.
    ///
    /// # Returns
    /// A new SBMLDocument instance with the default SBML level and version, and FBC package.
    fn default() -> Self {
        Self::new(3, 2, vec![Package::Fbc(1).into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::SBMLErrorSeverity;

    use super::*;

    #[test]
    fn test_sbmldoc_new() {
        let doc = SBMLDocument::default();
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
        let doc = SBMLDocument::default();
        doc.create_model("test");

        let model = doc.model().expect("Model not found");
        assert_eq!(model.id(), "test");
    }

    #[test]
    fn test_sbmldoc_to_xml_string() {
        let doc = SBMLDocument::default();
        doc.create_model("test");

        let xml_string = doc.to_xml_string();
        assert!(!xml_string.is_empty());
    }

    #[test]
    fn test_sbmldoc_check_consistency() {
        let doc = SBMLDocument::default();
        let error_log = doc.check_consistency();
        assert!(error_log.valid);
    }

    #[test]
    fn test_sbmldoc_check_consistency_invalid() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("model");

        // Lets add a species without a compartment
        model
            .build_species("some")
            .initial_concentration(-10.0)
            .build();

        let error_log = doc.check_consistency();
        let errors = error_log
            .errors
            .iter()
            .filter(|e| e.severity == SBMLErrorSeverity::Error)
            .count();

        assert!(!error_log.valid);
        assert_eq!(errors, 1);

        // Check that the error log contains the correct number of errors
    }

    #[test]
    fn test_sbmldoc_check_consistency_warning() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("model");

        // Lets create a parameter with nothing
        // This should throw a couple of warnings
        model.build_parameter("test").build();

        let error_log = doc.check_consistency();
        let warnings = error_log
            .errors
            .iter()
            .filter(|e| e.severity == SBMLErrorSeverity::Warning)
            .count();

        assert!(error_log.valid);
        assert_eq!(warnings, 4);
    }

    #[test]
    fn test_sbmldoc_new_with_packages() {
        let doc = SBMLDocument::new(3, 2, vec![Package::Fbc(1).into()]);
        assert_eq!(doc.level(), 3);
        assert_eq!(doc.version(), 2);
        assert!(doc.plugins().contains(&"fbc".to_string()));
    }

    #[test]
    fn test_sbmldoc_new_without_packages() {
        let doc = SBMLDocument::new(3, 2, vec![]);
        println!("{:?}", doc.plugins());
        assert!(!doc.plugins().is_empty());
    }
}
