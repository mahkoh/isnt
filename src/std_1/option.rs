// This file was generated

mod option_private { pub trait Sealed<T> { } }

/// Extension for [`Option`](Option)
pub trait IsntOptionExt<T>: option_private::Sealed<T> {
    /// The negation of [`is_some`](Option::is_some)
    #[must_use]
    fn is_not_some(&self) -> bool;
    /// The negation of [`is_none`](Option::is_none)
    #[must_use]
    fn is_not_none(&self) -> bool;
}

impl<T> option_private::Sealed<T> for Option<T> { }

impl<T> IsntOptionExt<T> for Option<T> {
    #[inline]
    fn is_not_some(&self) -> bool {
        !self.is_some()
    }

    #[inline]
    fn is_not_none(&self) -> bool {
        !self.is_none()
    }
}
