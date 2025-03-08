//! Internal trait for creating SBML wrapper types from raw pointers
//!
//! This trait is used internally by the SBML wrapper types to create safe Rust wrappers
//! around the raw libSBML C++ pointers. It provides a consistent interface for converting
//! from unsafe pointers to safe wrapper types.
//!
//! The trait is only used within the crate and is not exposed publicly. Each wrapper type
//! like Species, Parameter, etc implements this trait to handle creation from raw pointers
//! returned by the libSBML C++ API.

/// Internal trait for creating wrapper types from raw pointers.
///
/// This trait provides a consistent way to create safe wrapper types from raw libSBML pointers.
/// It is implemented by the various SBML wrapper types to handle conversion from unsafe C++
/// pointers to safe Rust types.
///
/// # Type Parameters
/// * `T` - The raw pointer type from libSBML (e.g. sbmlcxx::Species)
pub(crate) trait FromPtr<T> {
    /// Creates a new wrapper instance from a raw pointer.
    ///
    /// # Arguments
    /// * `ptr` - Raw pointer to the libSBML object
    ///
    /// # Returns
    /// A new wrapper instance that safely manages the raw pointer
    fn from_ptr(ptr: *mut T) -> Self;
}
