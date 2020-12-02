// This file was generated

mod weak_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for [`Weak`](std::rc::Weak)
pub trait IsntWeakExt<T: ?Sized>: weak_private::Sealed<T> {
    /// The negation of [`ptr_eq`](std::rc::Weak::ptr_eq)
    #[must_use]
    fn not_ptr_eq(&self, other: &std::rc::Weak<T>) -> bool;
}

impl<T: ?Sized> weak_private::Sealed<T> for std::rc::Weak<T> { }

impl<T: ?Sized> IsntWeakExt<T> for std::rc::Weak<T> {
    #[inline]
    fn not_ptr_eq(&self, other: &std::rc::Weak<T>) -> bool {
        !self.ptr_eq(other)
    }
}
