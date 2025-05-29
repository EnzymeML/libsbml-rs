//! This module provides a safe Rust interface to the libSBML SBMLReader class.
//!
//! The SBMLReader class is responsible for reading SBML models from XML strings or files.
//! It provides functionality to parse SBML documents and create in-memory representations
//! that can be manipulated using the rest of the library.
//!
//! This wrapper provides safe access to the underlying C++ libSBML SBMLReader class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin};

use autocxx::WithinBox;
use cxx::{let_cxx_string, UniquePtr};

use crate::{sbmlcxx, sbmldoc::SBMLDocument};

/// A safe wrapper around the libSBML SBMLReader class.
///
/// This struct maintains a reference to the underlying C++ SBMLReader object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
/// It provides methods to read SBML documents from various sources.
pub struct SBMLReader(RefCell<Pin<Box<sbmlcxx::SBMLReader>>>);

impl SBMLReader {
    /// Creates a new SBMLReader instance.
    ///
    /// # Returns
    /// A new SBMLReader instance ready to parse SBML documents
    pub fn new() -> Self {
        let reader = sbmlcxx::SBMLReader::new().within_box();
        Self(RefCell::new(reader))
    }

    /// Reads an SBML document from an XML string.
    ///
    /// # Arguments
    /// * `xml` - A string containing valid SBML XML
    ///
    /// # Returns
    /// An SBMLDocument instance containing the parsed model
    pub fn from_xml_string(xml: &str) -> SBMLDocument {
        let reader = Self::new();
        // Create an owned String to ensure the data persists
        let owned_xml = xml.to_string();
        let_cxx_string!(xml_cxx = owned_xml);
        let ptr = unsafe {
            UniquePtr::from_raw(reader.0.borrow_mut().as_mut().readSBMLFromString(&xml_cxx))
        };
        SBMLDocument::from_unique_ptr(ptr)
    }
}

impl Default for SBMLReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::errors::LibSBMLError;

    use super::*;

    #[test]
    fn test_read_sbml_string() {
        let doc = SBMLReader::from_xml_string(include_str!("../tests/data/example.xml"));
        assert_eq!(doc.model().expect("Model not found").id(), "example");

        // There are two species
        let list_of_species = doc.model().expect("Model not found").list_of_species();
        assert_eq!(list_of_species.len(), 2);

        // There is only one compartment
        let list_of_compartments = doc.model().expect("Model not found").list_of_compartments();
        assert_eq!(list_of_compartments.len(), 1);

        // There are two unit definitions
        let list_of_unit_definitions = doc
            .model()
            .expect("Model not found")
            .list_of_unit_definitions();
        assert_eq!(list_of_unit_definitions.len(), 2);

        // There is one reaction
        let list_of_reactions = doc.model().expect("Model not found").list_of_reactions();
        assert_eq!(list_of_reactions.len(), 1);

        // There is no parameter
        let list_of_parameters = doc.model().expect("Model not found").list_of_parameters();
        assert_eq!(list_of_parameters.len(), 0);
    }

    #[test]
    fn test_read_sbml_file_rules_only() {
        // This test uses an "external" function to ensure that returning an SBMLDocument
        // from a function works as expected.
        let doc = read_sbml_file(&PathBuf::from("tests/data/odes_example_test.xml")).unwrap();

        let model = doc.model().expect("Model not found");

        // There are 5 unit definitions
        let list_of_unit_definitions = model.list_of_unit_definitions();
        assert_eq!(list_of_unit_definitions.len(), 5);

        // There is 1 compartment
        let list_of_compartments = model.list_of_compartments();
        assert_eq!(list_of_compartments.len(), 1);

        // There is 1 reaction
        let list_of_reactions = model.list_of_reactions();
        assert_eq!(list_of_reactions.len(), 0);

        // There are 4 species
        let list_of_species = model.list_of_species();
        assert_eq!(list_of_species.len(), 4);

        // There are 3 parameters
        let list_of_parameters = model.list_of_parameters();
        assert_eq!(list_of_parameters.len(), 3);

        // There is 1 rate rule
        let list_of_rate_rules = model.list_of_rate_rules();
        assert_eq!(list_of_rate_rules.len(), 1);

        // There are 0 assignment rules
        let list_of_assignment_rules = model.list_of_assignment_rules();
        assert_eq!(list_of_assignment_rules.len(), 0);
    }

    fn read_sbml_file(path: &PathBuf) -> Result<SBMLDocument, LibSBMLError> {
        let xml = std::fs::read_to_string(path).unwrap();
        Ok(SBMLReader::from_xml_string(&xml))
    }
}
