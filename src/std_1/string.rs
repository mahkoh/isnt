// This file was generated

mod string_private { pub trait Sealed { } }

/// Extension for [`String`](std::string::String)
pub trait IsntStringExt: string_private::Sealed {
    /// The negation of [`is_empty`](std::string::String::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl string_private::Sealed for std::string::String { }

impl IsntStringExt for std::string::String {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}
