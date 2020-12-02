// This file was generated

mod formatter_private { pub trait Sealed<'a> { } }

/// Extension for [`Formatter`](std::fmt::Formatter)
pub trait IsntFormatterExt<'a>: formatter_private::Sealed<'a> {
    /// The negation of [`sign_plus`](std::fmt::Formatter::sign_plus)
    #[must_use]
    fn not_sign_plus(&self) -> bool;
    /// The negation of [`sign_minus`](std::fmt::Formatter::sign_minus)
    #[must_use]
    fn not_sign_minus(&self) -> bool;
    /// The negation of [`alternate`](std::fmt::Formatter::alternate)
    #[must_use]
    fn not_alternate(&self) -> bool;
    /// The negation of [`sign_aware_zero_pad`](std::fmt::Formatter::sign_aware_zero_pad)
    #[must_use]
    fn not_sign_aware_zero_pad(&self) -> bool;
}

impl<'a> formatter_private::Sealed<'a> for std::fmt::Formatter<'a> { }

impl<'a> IsntFormatterExt<'a> for std::fmt::Formatter<'a> {
    #[inline]
    fn not_sign_plus(&self) -> bool {
        !self.sign_plus()
    }

    #[inline]
    fn not_sign_minus(&self) -> bool {
        !self.sign_minus()
    }

    #[inline]
    fn not_alternate(&self) -> bool {
        !self.alternate()
    }

    #[inline]
    fn not_sign_aware_zero_pad(&self) -> bool {
        !self.sign_aware_zero_pad()
    }
}
