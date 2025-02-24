/// Creates a pinned reference from a raw pointer.
///
/// This macro takes a raw pointer and type, and creates a pinned mutable reference
/// to that type. It is used internally to safely work with C++ objects that must
/// not be moved in memory.
///
/// # Arguments
/// * `$name` - The raw pointer to convert
/// * `$type` - The type to convert the pointer to
///
/// # Safety
/// This macro uses unsafe code to create the reference and pin it. The caller must
/// ensure that:
/// - The pointer is valid and properly aligned
/// - The pointer points to an initialized value of the specified type
/// - The lifetime of the reference does not outlive the pointed-to data
#[macro_export]
macro_rules! pin_ptr {
    ($name:ident, $type:ty) => {{
        let ref_ptr: &mut $type = unsafe { &mut *$name };
        unsafe { Pin::new_unchecked(ref_ptr) }
    }};
}
