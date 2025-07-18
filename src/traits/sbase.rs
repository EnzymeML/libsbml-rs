use crate::sbmlcxx;

use super::inner::Inner;

pub(crate) trait SBase<'a, T>: Inner<'a, T> {
    /// Returns a pinned reference to the underlying SBase object.
    ///
    /// This is useful when you need to pass a pinned reference to C++ code.
    #[allow(clippy::mut_from_ref)]
    fn base(&self) -> std::pin::Pin<&mut sbmlcxx::SBase>;
}
