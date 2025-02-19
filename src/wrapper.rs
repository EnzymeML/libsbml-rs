//! Provides a wrapper struct for deserializing annotations in SBML models.
//!
//! This module defines a generic wrapper that allows for flexible deserialization
//! of annotations with custom types in SBML-related data structures.

use serde::Deserialize;

/// A generic wrapper struct for deserializing XML annotations.
///
/// This struct allows for flexible deserialization of annotations by wrapping
/// a generic type `T` with a specific XML structure. It is particularly useful
/// when working with serialized metadata in SBML models.
///
/// # Type Parameters
/// * `T` - The type of the annotation content to be deserialized
///
/// # Serde Configuration
/// * Renames the root XML element to "annotation"
/// * Uses "$value" to capture the inner content
///
/// # Examples
/// ```
/// // Deserialize a custom annotation type
/// #[derive(Deserialize)]
/// struct MyAnnotation {
///     key: String,
///     value: i32,
/// }
///
/// // The Wrapper allows deserialization of MyAnnotation from XML
/// let annotation: Wrapper<MyAnnotation> = from_str(xml_string)?;
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename = "annotation")]
pub(crate) struct Wrapper<T> {
    /// The actual annotation content
    ///
    /// Uses a special serde rename to capture the inner XML value
    #[serde(rename = "$value")]
    pub(crate) annotation: T,
}
