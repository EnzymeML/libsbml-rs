//! Provides a wrapper struct for deserializing annotations in SBML models.
//!
//! This module defines a generic wrapper that allows for flexible deserialization
//! of annotations with custom types in SBML-related data structures.

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::marker::PhantomData;

/// A generic wrapper struct for deserializing XML annotations.
///
/// This struct allows for flexible deserialization of annotations by wrapping
/// a generic type `T` with a specific XML structure. It is particularly useful
/// when working with serialized metadata in SBML models.
///
/// The custom deserializer iterates through all child elements within the
/// `<annotation>` tag and attempts to deserialize each one into type `T`.
/// If multiple elements can be parsed into `T`, the first successful one is used.
/// Elements that cannot be parsed into `T` are silently ignored.
///
/// # Type Parameters
/// * `T` - The type of the annotation content to be deserialized
///
/// # Behavior
/// * Expects XML with root element named "annotation"
/// * Iterates through all child elements
/// * Attempts to deserialize each child element into type `T`
/// * Returns the first successful match
/// * Ignores elements that cannot be parsed into `T`
///
/// # Example
/// ```xml
/// <annotation>
///   <test>some_value</test>
///   <other_field>ignored</other_field>
///   <name>also_ignored</name>
/// </annotation>
/// ```
///
/// When deserializing into `Wrapper<TestStruct>` where `TestStruct` has a `test` field,
/// only the `<test>` element would be successfully parsed, while others are ignored.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "annotation")]
pub(crate) struct Wrapper<T> {
    /// The actual annotation content
    pub(crate) annotation: T,
}

impl<'de, T> Deserialize<'de> for Wrapper<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WrapperVisitor<T> {
            marker: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for WrapperVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Wrapper<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an annotation element with parseable content")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut last_error: Option<String> = None;

                // Iterate through all key-value pairs
                while let Some(key) = map.next_key::<String>()? {
                    // Try to deserialize the next value into T
                    // This handles both single values and nested structures
                    match map.next_value::<T>() {
                        Ok(parsed_value) => {
                            // Successfully parsed this element into T
                            return Ok(Wrapper {
                                annotation: parsed_value,
                            });
                        }
                        Err(err) => {
                            // This element couldn't be parsed into T, store error and continue
                            last_error = Some(format!("Failed to parse '{}': {}", key, err));
                            continue;
                        }
                    }
                }

                // If we get here, no element could be parsed into T
                match last_error {
                    Some(err) => Err(de::Error::custom(err)),
                    None => Err(de::Error::custom(
                        "no elements found that could be parsed into the target type",
                    )),
                }
            }
        }

        // Use a map deserializer since XML elements are treated as key-value pairs
        deserializer.deserialize_map(WrapperVisitor {
            marker: PhantomData,
        })
    }
}

impl<T> Wrapper<T> {
    /// Creates a new wrapper with the given annotation content.
    #[allow(dead_code)]
    pub(crate) fn new(annotation: T) -> Self {
        Self { annotation }
    }

    /// Gets a reference to the annotation content.
    #[allow(dead_code)]
    pub(crate) fn get(&self) -> &T {
        &self.annotation
    }

    /// Consumes the wrapper and returns the annotation content.
    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> T {
        self.annotation
    }
}
