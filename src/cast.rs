use std::pin::Pin;

/// Safely upcasts a pinned reference from a derived type to its superclass type.
///
/// # Safety
///
/// This function is unsafe because it:
/// 1. Takes a pinned reference and temporarily converts it to a raw pointer
/// 2. Performs pointer casting between types
/// 3. Creates a new pinned reference from the cast pointer
///
/// The caller must ensure that:
/// - Type `D` is actually a derived class of superclass `S`
/// - The memory layout is compatible for the upcast
/// - The pinning guarantees are maintained
///
/// # Arguments
///
/// * `derived` - A pinned mutable reference to the derived type
///
/// # Returns
///
/// A pinned mutable reference to the superclass type
///
/// # Type Parameters
///
/// * `D` - The derived class type
/// * `S` - The superclass type
///
/// # Implementation Details
///
/// This function works by:
/// 1. Converting the pinned reference to a raw pointer using `Pin::into_inner_unchecked`
/// 2. Casting the raw pointer from derived type to superclass type
/// 3. Creating a new pinned reference from the cast pointer
///
/// This is typically used when working with C++ inheritance hierarchies through FFI.
pub(crate) unsafe fn upcast<'a, D, S>(derived: *mut D) -> Pin<&'a mut S> {
    std::pin::Pin::new_unchecked(&mut *derived.cast::<S>())
}
