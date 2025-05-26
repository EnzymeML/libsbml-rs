//! Trait for converting strings into string references with appropriate lifetimes
//!
//! This module provides functionality to convert owned Strings and string slices
//! into string references with proper lifetime management. This is useful
//! when working with APIs that require string references.
//!
//! # Examples
//! ```
//! use sbml::traits::intoid::IntoId;
//!
//! let owned = String::from("species1");
//! let str_ref = owned.into_id(); // Borrows from owned
//!
//! let slice = "species2";
//! let str_ref = slice.into_id(); // Simply returns the slice
//! ```

/// Trait for converting a type into an owned string identifier
pub trait IntoId {
    /// Converts self into an owned string identifier
    fn into_id(self) -> String;
}

impl IntoId for String {
    fn into_id(self) -> String {
        self
    }
}

impl IntoId for &str {
    fn into_id(self) -> String {
        self.to_string()
    }
}

impl IntoId for &String {
    fn into_id(self) -> String {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::into_id;

    use super::*;

    #[test]
    fn test_into_id() {
        // Test with a String reference
        let owned = String::from("species1");
        let str_ref = (&owned).into_id();
        assert_eq!(str_ref, "species1");

        // Test with a string slice
        let slice = "species2";
        let str_ref = slice.into_id();
        assert_eq!(str_ref, "species2");

        // Test with a struct
        struct TestStruct {
            id: String,
        }

        impl TestStruct {
            fn id(&self) -> String {
                self.id.clone()
            }
        }

        into_id!(TestStruct, id);

        let test_struct = TestStruct {
            id: String::from("species3"),
        };
        let str_ref = test_struct.id();
        assert_eq!(str_ref, "species3");
    }
}
