// This file was generated

mod os_str_private { pub trait Sealed { } }

/// Extension for [`OsStr`](std::ffi::OsStr)
pub trait IsntOsStrExt: os_str_private::Sealed {
    /// The negation of [`is_empty`](std::ffi::OsStr::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl os_str_private::Sealed for std::ffi::OsStr { }

impl IsntOsStrExt for std::ffi::OsStr {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}
