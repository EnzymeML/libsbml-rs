use std::{cell::RefCell, pin::Pin};

/// A trait for types that provide access to their inner pinned data.
///
/// This trait is used to provide a consistent interface for accessing the inner
/// pinned data of wrapper types in the SBML library. The inner data is typically
/// a pointer to a C++ object that needs to be pinned in memory.
///
/// # Type Parameters
///
/// * `T` - The type of the inner pinned data
///
/// # Safety
///
/// Implementations must ensure that the returned pinned reference maintains
/// the pinning guarantees - the data must not be moved after it is pinned.
pub(crate) trait Inner<'a, T> {
    /// Returns a pinned mutable reference to the inner data.
    ///
    /// # Returns
    ///
    /// A pinned mutable reference to the inner data of type `T`
    fn inner(&self) -> &RefCell<Pin<&'a mut T>>;
}
