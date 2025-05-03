//! Annotation handling for SBML elements
//!
//! This module provides functionality for handling annotations in SBML elements through
//! the Annotation trait. Annotations allow attaching additional metadata or information
//! to SBML elements like Models, Species, and Compartments.
//!
//! The module supports both raw string annotations and structured data through serde
//! serialization/deserialization. This enables storing complex metadata in a type-safe way.
//!
//! # Example
//! ```no_run
//! use serde::{Serialize, Deserialize};
//! use libsbml::prelude::*;
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyAnnotation {
//!     name: String,
//!     value: f64
//! }
//!
//! let doc = SBMLDocument::new(3, 2);
//! let model = doc.create_model("example");
//!
//! // Set annotation using a struct
//! let annotation = MyAnnotation {
//!     name: "test".into(),
//!     value: 1.23
//! };
//! model.set_annotation_serde(&annotation);
//!
//! // Get annotation as struct
//! let retrieved: MyAnnotation = model.get_annotation_serde().unwrap();
//! ```

use std::error::Error;

use quick_xml::{DeError, SeError};
use serde::{Deserialize, Serialize};

/// Trait for handling annotations in SBML elements.
///
/// This trait provides functionality for getting and setting annotations on SBML elements
/// like Models, Species, and Compartments. Annotations can be used to store additional
/// metadata or information about the elements in a structured format.
///
/// The trait supports both raw string annotations and serializable/deserializable
/// data structures through serde.
pub trait Annotation {
    /// Gets the raw annotation string for this element.
    ///
    /// # Returns
    /// The annotation as a String
    fn get_annotation(&self) -> String;

    /// Sets a raw string annotation for this element.
    ///
    /// # Arguments
    /// * `annotation` - The string annotation to set
    fn set_annotation(&self, annotation: &str) -> Result<(), Box<dyn Error>>;

    /// Sets an annotation using a serializable data structure.
    ///
    /// This method will serialize the provided data structure into a string
    /// format before setting it as the annotation.
    ///
    /// # Arguments
    /// * `annotation` - The serializable data structure to use as annotation
    fn set_annotation_serde<T: Serialize>(&self, annotation: &T) -> Result<(), SeError>;

    /// Gets the annotation as a deserializable data structure.
    ///
    /// This method will attempt to deserialize the annotation string into
    /// the specified type.
    ///
    /// # Type Parameters
    /// * `T` - The type to deserialize the annotation into
    ///
    /// # Returns
    /// A Result containing either the deserialized annotation or a deserialization error
    fn get_annotation_serde<T: for<'de> Deserialize<'de>>(&self) -> Result<T, DeError>;
}
