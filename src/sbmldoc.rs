//! SBML Document handling
//!
//! This module provides a safe Rust wrapper around libSBML's SBMLDocument class.
//! The Systems Biology Markup Language (SBML) is an XML-based format for representing
//! computational models in systems biology. An SBMLDocument is the root container
//! for all SBML content.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use autocxx::WithinUniquePtr;
use cxx::{let_cxx_string, UniquePtr};
use std::pin::Pin;

use crate::{
    cast::upcast,
    model::Model,
    namespaces::SBMLNamespaces,
    packages::{Package, PackageSpec},
    pin_const_ptr, pin_ptr,
    prelude::SBMLErrorLog,
    sbmlcxx,
    traits::fromptr::FromPtr,
};

/// A wrapper around libSBML's SBMLDocument class that provides a safe Rust interface.
///
/// The SBMLDocument is the top-level container for an SBML model and associated data.
/// It maintains the SBML level and version, and contains a single optional Model.
pub struct SBMLDocument {
    /// The underlying libSBML document, wrapped in RefCell to allow interior mutability
    document: RefCell<UniquePtr<sbmlcxx::SBMLDocument>>,
}

impl SBMLDocument {
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
    pub(crate) fn from_unique_ptr(ptr: UniquePtr<sbmlcxx::SBMLDocument>) -> SBMLDocument {
        // Wrap the pointer in a RefCell
        let document = RefCell::new(ptr);

        SBMLDocument { document }
    }

    /// Returns a reference to the underlying libSBML document.
    ///
    /// This is primarily for internal use by other parts of the API.
    pub(crate) fn inner(&self) -> &RefCell<UniquePtr<sbmlcxx::SBMLDocument>> {
        &self.document
    }

    /// Returns the XML namespaces defined in this SBML document.
    ///
    /// This method retrieves all namespace prefix-URI pairs that are defined
    /// in the document's XML namespace declarations. This includes the core
    /// SBML namespace as well as any package extension namespaces.
    ///
    /// # Returns
    /// A HashMap where keys are namespace prefixes and values are namespace URIs.
    /// An empty prefix string represents the default namespace.
    pub fn namespaces(&self) -> HashMap<String, String> {
        let ns_ptr = self.inner().borrow_mut().getNamespaces();
        let namespaces = pin_ptr!(ns_ptr, sbmlcxx::XMLNamespaces);

        let mut ns_map = HashMap::new();
        let num_namespaces = namespaces.getNumNamespaces().into();
        for i in 0..num_namespaces {
            let prefix = namespaces.getPrefix(i.into());
            let uri = namespaces.getURI(i.into());
            ns_map.insert(prefix.to_string(), uri.to_string());
        }

        ns_map
    }

    /// Adds a namespace declaration to this SBML document.
    ///
    /// This method adds a new XML namespace prefix-URI pair to the document's
    /// namespace declarations. This is useful when working with SBML package
    /// extensions that require specific namespace declarations.
    ///
    /// # Arguments
    /// * `prefix` - The namespace prefix to associate with the URI
    /// * `uri` - The namespace URI to be declared
    pub fn add_namespace(&self, prefix: &str, uri: &str) {
        let ns_ptr = self.inner().borrow_mut().getNamespaces();
        let mut namespaces = pin_ptr!(ns_ptr, sbmlcxx::XMLNamespaces);

        let_cxx_string!(uri = uri);
        namespaces.as_mut().add(&uri, prefix);
    }

    /// Removes a namespace declaration from this SBML document.
    ///
    /// This method removes an XML namespace prefix-URI pair from the document's
    /// namespace declarations. This is useful when you need to clean up or modify
    /// the namespace declarations in an SBML document.
    ///
    /// # Arguments
    /// * `prefix` - The namespace prefix to remove from the document
    ///
    /// # Returns
    /// Result indicating success or containing an error message if the removal failed
    pub fn remove_namespace(&self, prefix: &str) -> Result<(), String> {
        let ns_ptr = self.inner().borrow_mut().getNamespaces();
        let mut namespaces = pin_ptr!(ns_ptr, sbmlcxx::XMLNamespaces);

        let_cxx_string!(prefix_cpp = prefix);
        let res = namespaces.as_mut().remove1(&prefix_cpp);

        match res.0 {
            n if n < 0 => Err(format!(
                "The namespace '{prefix}' could not be removed. The prefix may not be present."
            )),
            _ => Ok(()),
        }
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
    pub fn create_model<'a>(&'a self, id: &str) -> Rc<Model<'a>> {
        Rc::new(Model::new(self, id))
    }

    /// Returns a reference to the Model if one exists.
    pub fn model<'a>(&'a self) -> Option<Rc<Model<'a>>> {
        // Check if a model exists in the document
        let has_model = self.document.borrow_mut().as_mut()?.isSetModel();

        if has_model {
            Some(Rc::new(Model::from_ptr(
                self.document.borrow_mut().as_mut()?.getModel1(),
            )))
        } else {
            None
        }
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

impl std::fmt::Debug for SBMLDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("SBMLDocument");
        ds.field("level", &self.level());
        ds.field("version", &self.version());
        ds.field("model", &self.model());
        ds.finish()
    }
}

impl Default for SBMLDocument {
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

    #[test]
    fn test_sbmldoc_lifetime_changes() {
        // Test that we can create a document and model without lifetime issues
        let doc = SBMLDocument::default();
        let model = doc.create_model("test_model");

        // Test that we can create species and other components
        let species = model.create_species("test_species");
        assert_eq!(species.id(), "test_species");

        // Test that we can get the model back
        let retrieved_model = doc.model().expect("Model should exist");
        assert_eq!(retrieved_model.id(), "test_model");

        // Test that the document doesn't have lifetime parameters
        let _xml = doc.to_xml_string();
        assert!(!_xml.is_empty());
    }

    #[test]
    fn test_retrieve_namespaces() {
        let doc = SBMLDocument::default();
        assert!(!doc.namespaces().is_empty());
        assert!(doc.namespaces().contains_key(""));
        assert!(doc.namespaces().contains_key("fbc"));
    }

    #[test]
    fn test_add_namespace() {
        let doc = SBMLDocument::default();
        doc.add_namespace("enzymeml", "https://www.enzymeml.org/version2");

        // Check if the ns has been added
        let namespaces = doc.namespaces();
        assert!(namespaces.contains_key("enzymeml"));
        assert_eq!(namespaces["enzymeml"], "https://www.enzymeml.org/version2");
    }

    #[test]
    fn test_remove_namespace() {
        let doc = SBMLDocument::default();
        doc.add_namespace("enzymeml", "https://www.enzymeml.org/version2");

        doc.remove_namespace("enzymeml")
            .expect("Could not remove namespace");

        // Check if the ns has been removed
        let namespaces = doc.namespaces();
        assert!(!namespaces.contains_key("enzymeml"));
    }

    #[test]
    #[should_panic]
    fn test_remove_namespace_non_existent() {
        let doc = SBMLDocument::default();

        doc.remove_namespace("enzymeml")
            .expect("Could not remove namespace");
    }
}
