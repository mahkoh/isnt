// This file was generated

mod result_private { pub trait Sealed<T, E> { } }

/// Extension for [`Result`](Result)
pub trait IsntResultExt<T, E>: result_private::Sealed<T, E> {
    /// The negation of [`is_ok`](Result::is_ok)
    #[must_use]
    fn is_not_ok(&self) -> bool;
    /// The negation of [`is_err`](Result::is_err)
    #[must_use]
    fn is_not_err(&self) -> bool;
}

impl<T, E> result_private::Sealed<T, E> for Result<T, E> { }

impl<T, E> IsntResultExt<T, E> for Result<T, E> {
    #[inline]
    fn is_not_ok(&self) -> bool {
        !self.is_ok()
    }

    #[inline]
    fn is_not_err(&self) -> bool {
        !self.is_err()
    }
}
