// This file was generated

mod non_null_private { pub trait Sealed<T> { } }

/// Extension for [`NonNull`](std::ptr::NonNull)
pub trait IsntNonNullExt<T>: non_null_private::Sealed<T> {
    /// The negation of [`is_aligned`](std::ptr::NonNull::is_aligned)
    #[must_use]
    fn is_not_aligned(self) -> bool;
}

impl<T> non_null_private::Sealed<T> for std::ptr::NonNull<T> { }

impl<T> IsntNonNullExt<T> for std::ptr::NonNull<T> {
    #[inline]
    fn is_not_aligned(self) -> bool {
        !self.is_aligned()
    }
}

mod non_null_slice_private { pub trait Sealed<T> { } }

/// Extension for [`NonNull`](std::ptr::NonNull)
pub trait IsntNonNullSliceExt<T>: non_null_slice_private::Sealed<T> {
    /// The negation of [`is_empty`](std::ptr::NonNull::is_empty)
    #[must_use]
    fn is_not_empty(self) -> bool;
}

impl<T> non_null_slice_private::Sealed<T> for std::ptr::NonNull<[T]> { }

impl<T> IsntNonNullSliceExt<T> for std::ptr::NonNull<[T]> {
    #[inline]
    fn is_not_empty(self) -> bool {
        !self.is_empty()
    }
}
