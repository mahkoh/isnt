// This file was generated

mod duration_private { pub trait Sealed { } }

/// Extension for [`Duration`](std::time::Duration)
pub trait IsntDurationExt: duration_private::Sealed {
    /// The negation of [`is_zero`](std::time::Duration::is_zero)
    #[must_use]
    fn is_not_zero(&self) -> bool;
}

impl duration_private::Sealed for std::time::Duration { }

impl IsntDurationExt for std::time::Duration {
    #[inline]
    fn is_not_zero(&self) -> bool {
        !self.is_zero()
    }
}
