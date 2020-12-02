// This file was generated

mod vec_private { pub trait Sealed<T> { } }

/// Extension for [`Vec`](Vec)
pub trait IsntVecExt<T>: vec_private::Sealed<T> {
    /// The negation of [`is_empty`](Vec::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<T> vec_private::Sealed<T> for Vec<T> { }

impl<T> IsntVecExt<T> for Vec<T> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}
