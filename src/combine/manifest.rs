//! The manifest module provides functionality for working with COMBINE archive manifests.
//!
//! A COMBINE archive is a ZIP container format that bundles together multiple files used in
//! computational modeling in biology. The manifest file describes the contents of the archive,
//! including their locations and formats.
//!
//! This module provides:
//! - Serialization and deserialization of OMEX manifest files
//! - Types for representing manifest data
//! - Support for common formats used in systems biology

use std::{fmt::Display, str::FromStr};

use quick_xml::SeError;
use serde::{Deserialize, Serialize};

use super::error::CombineArchiveError;

/// Represents an OMEX manifest file for COMBINE archives
///
/// An OMEX manifest describes the contents of a COMBINE archive, including
/// the location, format, and role of each file in the archive.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "omexManifest")]
pub struct OmexManifest {
    /// XML namespace for OMEX manifest
    ///
    /// The standard namespace is "http://identifiers.org/combine.specifications/omex-manifest"
    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    /// List of content entries in the manifest
    ///
    /// Each content entry describes a file within the COMBINE archive.
    #[serde(rename = "content")]
    pub content: Vec<Content>,
}

/// Represents a content entry in the OMEX manifest
///
/// Each content entry describes a single file within the COMBINE archive,
/// including its location, format, and whether it's the master file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Content {
    /// Location/path of the content file
    ///
    /// This is typically a relative path within the archive.
    #[serde(rename = "@location")]
    pub location: String,

    /// Format identifier (usually a URI)
    ///
    /// Identifies the format of the file, typically using a URI from identifiers.org.
    /// For example, SBML files use "http://identifiers.org/combine.specifications/sbml".
    #[serde(rename = "@format")]
    pub format: String,

    /// Whether this content is the master file
    ///
    /// The master file is the primary file that should be processed first
    /// when working with the archive.
    #[serde(rename = "@master")]
    pub master: bool,
}

impl Default for OmexManifest {
    /// Creates a default OMEX manifest with the standard namespace and an empty content list
    fn default() -> Self {
        Self {
            xmlns: "http://identifiers.org/combine.specifications/omex-manifest".to_string(),
            content: Vec::new(),
        }
    }
}

impl OmexManifest {
    /// Create a new OMEX manifest with default namespace
    ///
    /// This creates an empty manifest with the standard OMEX namespace.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a content entry to the manifest
    ///
    /// # Arguments
    ///
    /// * `location` - The location/path of the file within the archive
    /// * `format` - The format identifier for the file
    /// * `master` - Whether this file is the master file
    pub fn add_entry(
        &mut self,
        location: impl Into<String>,
        format: impl Into<String>,
        master: bool,
    ) -> Result<(), CombineArchiveError> {
        // First check if the there is already an entry with the same location
        let location = location.into();
        if self.content.iter().any(|c| c.location == location) {
            return Err(CombineArchiveError::LocationAlreadyExists(location));
        }

        self.content.push(Content {
            location,
            format: format.into(),
            master,
        });

        Ok(())
    }

    /// Serialize the manifest to XML string
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The serialized XML string
    /// * `Err(SeError)` - Error during serialization
    pub fn to_xml(&self) -> Result<String, SeError> {
        quick_xml::se::to_string(self)
    }

    /// Deserialize the manifest from XML string
    ///
    /// # Arguments
    ///
    /// * `xml` - The XML string to deserialize
    ///
    /// # Returns
    ///
    /// * `Ok(OmexManifest)` - The deserialized manifest
    /// * `Err(DeError)` - Error during deserialization
    pub fn from_xml(xml: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(xml)
    }

    pub fn has_location(&self, location: &str) -> bool {
        self.content.iter().any(|c| c.location == location)
    }

    pub fn has_format(&self, format: impl Into<String>) -> bool {
        let format = format.into();
        self.content.iter().any(|c| c.format == format)
    }

    pub fn master_file(&self) -> Option<&Content> {
        self.content.iter().find(|c| c.master)
    }
}

impl Content {
    /// Create a new content entry
    ///
    /// # Arguments
    ///
    /// * `location` - The location/path of the file within the archive
    /// * `format` - The format identifier for the file
    /// * `master` - Whether this file is the master file
    ///
    /// # Returns
    ///
    /// A new Content instance
    pub fn new(location: impl Into<String>, format: impl Into<String>, master: bool) -> Self {
        Self {
            location: location.into(),
            format: format.into(),
            master,
        }
    }
}

/// Enumeration of commonly used formats in COMBINE archives
///
/// This enum provides a type-safe way to work with well-known format identifiers.
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum KnownFormats {
    /// Systems Biology Markup Language (SBML)
    SBML,
    /// Simulation Experiment Description Markup Language (SED-ML)
    SEDML,
    /// Systems Biology Graphical Notation (SBGN)
    SBGN,
}

impl FromStr for KnownFormats {
    type Err = String;

    /// Parse a string into a KnownFormats value
    ///
    /// Accepts both full URIs and shorthand names.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse
    ///
    /// # Returns
    ///
    /// * `Ok(KnownFormats)` - The parsed format
    /// * `Err(String)` - Error message if the format is unknown
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http://identifiers.org/combine.specifications/sbml" | "sbml" => Ok(KnownFormats::SBML),
            "http://identifiers.org/combine.specifications/sed" | "sedml" => {
                Ok(KnownFormats::SEDML)
            }
            "http://identifiers.org/combine.specifications/sbgn" | "sbgn" => Ok(KnownFormats::SBGN),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

impl From<KnownFormats> for String {
    /// Convert a KnownFormats value to its URI string representation
    fn from(value: KnownFormats) -> Self {
        value.to_string()
    }
}

impl Display for KnownFormats {
    /// Format a KnownFormats value as its URI string representation
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnownFormats::SBML => write!(f, "http://identifiers.org/combine.specifications/sbml"),
            KnownFormats::SEDML => {
                write!(f, "http://identifiers.org/combine.specifications/sed")
            }
            KnownFormats::SBGN => write!(f, "http://identifiers.org/combine.specifications/sbgn"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_creation() {
        let mut manifest = OmexManifest::new();

        manifest
            .add_entry(
                ".",
                "http://identifiers.org/combine.specifications/omex",
                false,
            )
            .unwrap();
        manifest
            .add_entry(
                "./manifest.xml",
                "http://identifiers.org/combine.specifications/omex-manifest",
                false,
            )
            .unwrap();
        manifest
            .add_entry(
                "./model.xml",
                "http://identifiers.org/combine.specifications/sbml",
                true,
            )
            .unwrap();
        manifest
            .add_entry(
                "./data.tsv",
                "https://purl.org/NET/mediatypes/text/tab-separated-values",
                false,
            )
            .unwrap();

        assert_eq!(manifest.content.len(), 4);
        assert!(manifest.content[2].master);
        assert_eq!(manifest.content[0].location, ".");
    }

    #[test]
    fn test_xml_serialization() {
        let mut manifest = OmexManifest::new();
        manifest
            .add_entry(
                ".",
                "http://identifiers.org/combine.specifications/omex",
                false,
            )
            .unwrap();
        manifest
            .add_entry(
                "./manifest.xml",
                "http://identifiers.org/combine.specifications/omex-manifest",
                false,
            )
            .unwrap();
        manifest
            .add_entry(
                "./model.xml",
                "http://identifiers.org/combine.specifications/sbml",
                true,
            )
            .unwrap();
        manifest
            .add_entry(
                "./data.tsv",
                "https://purl.org/NET/mediatypes/text/tab-separated-values",
                false,
            )
            .unwrap();

        let xml = manifest.to_xml().expect("Failed to serialize to XML");
        assert!(xml.contains("omexManifest"));
        assert!(
            xml.contains("xmlns=\"http://identifiers.org/combine.specifications/omex-manifest\"")
        );
        assert!(xml.contains("master=\"true\""));
    }

    #[test]
    fn test_xml_deserialization() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<omexManifest xmlns="http://identifiers.org/combine.specifications/omex-manifest">
  <content location="." format="http://identifiers.org/combine.specifications/omex" master="false" />
  <content location="./manifest.xml" format="http://identifiers.org/combine.specifications/omex-manifest" master="false" />
  <content location="./model.xml" format="http://identifiers.org/combine.specifications/sbml" master="true" />
  <content location="./data.tsv" format="https://purl.org/NET/mediatypes/text/tab-separated-values" master="false" />
</omexManifest>"#;

        let manifest = OmexManifest::from_xml(xml).expect("Failed to deserialize from XML");

        assert_eq!(manifest.content.len(), 4);
        assert_eq!(
            manifest.xmlns,
            "http://identifiers.org/combine.specifications/omex-manifest"
        );
        assert!(manifest.content[2].master);
        assert_eq!(manifest.content[2].location, "./model.xml");
    }

    #[test]
    fn test_roundtrip_serialization() {
        let mut original = OmexManifest::new();
        original
            .add_entry(
                ".",
                "http://identifiers.org/combine.specifications/omex",
                false,
            )
            .unwrap();
        original
            .add_entry(
                "./model.xml",
                "http://identifiers.org/combine.specifications/sbml",
                true,
            )
            .unwrap();

        let xml = original.to_xml().expect("Failed to serialize");
        let deserialized = OmexManifest::from_xml(&xml).expect("Failed to deserialize");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_known_formats() {
        assert_eq!(KnownFormats::from_str("sbml"), Ok(KnownFormats::SBML));
        assert_eq!(KnownFormats::from_str("sedml"), Ok(KnownFormats::SEDML));
        assert_eq!(KnownFormats::from_str("sbgn"), Ok(KnownFormats::SBGN));
        assert_eq!(
            KnownFormats::from_str("unknown"),
            Err("Unknown format: unknown".to_string())
        );
    }

    #[test]
    fn test_known_formats_display() {
        assert_eq!(
            KnownFormats::SBML.to_string(),
            "http://identifiers.org/combine.specifications/sbml"
        );
        assert_eq!(
            KnownFormats::SEDML.to_string(),
            "http://identifiers.org/combine.specifications/sed"
        );
        assert_eq!(
            KnownFormats::SBGN.to_string(),
            "http://identifiers.org/combine.specifications/sbgn"
        );
    }

    #[test]
    fn test_add_content_from_known_formats() {
        let mut manifest = OmexManifest::new();
        manifest
            .add_entry("./sbml.xml", KnownFormats::SBML, false)
            .unwrap();
        assert_eq!(
            manifest.content[0].format,
            "http://identifiers.org/combine.specifications/sbml"
        );

        assert_eq!(manifest.content[0].location, "./sbml.xml");
        assert!(!manifest.content[0].master);

        manifest
            .add_entry("./sedml.xml", KnownFormats::SEDML, false)
            .unwrap();
        assert_eq!(
            manifest.content[1].format,
            "http://identifiers.org/combine.specifications/sed"
        );
        assert_eq!(manifest.content[1].location, "./sedml.xml");
        assert!(!manifest.content[1].master);

        manifest
            .add_entry("./sbgn.xml", KnownFormats::SBGN, false)
            .unwrap();
        assert_eq!(
            manifest.content[2].format,
            "http://identifiers.org/combine.specifications/sbgn"
        );
        assert_eq!(manifest.content[2].location, "./sbgn.xml");
        assert!(!manifest.content[2].master);
    }

    #[test]
    fn test_add_entry_duplicate_location() {
        let mut manifest = OmexManifest::new();
        manifest.add_entry(".", KnownFormats::SBML, false).unwrap();
        assert!(manifest.add_entry(".", KnownFormats::SBML, false).is_err());
    }

    #[test]
    fn test_has_location() {
        let mut manifest = OmexManifest::new();
        assert!(!manifest.has_location("."));
        manifest.add_entry(".", KnownFormats::SBML, false).unwrap();
        assert!(manifest.has_location("."));
    }

    #[test]
    fn test_has_format() {
        let mut manifest = OmexManifest::new();
        assert!(!manifest.has_format(KnownFormats::SBML));
        manifest.add_entry(".", KnownFormats::SBML, false).unwrap();
        assert!(manifest.has_format(KnownFormats::SBML));
    }
}
