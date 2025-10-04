// This file was generated

mod c_str_private { pub trait Sealed { } }

/// Extension for [`CStr`](std::ffi::CStr)
pub trait IsntCStrExt: c_str_private::Sealed {
    /// The negation of [`is_empty`](std::ffi::CStr::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl c_str_private::Sealed for std::ffi::CStr { }

impl IsntCStrExt for std::ffi::CStr {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

mod os_str_private { pub trait Sealed { } }

/// Extension for [`OsStr`](std::ffi::OsStr)
pub trait IsntOsStrExt: os_str_private::Sealed {
    /// The negation of [`is_empty`](std::ffi::OsStr::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
    /// The negation of [`is_ascii`](std::ffi::OsStr::is_ascii)
    #[must_use]
    fn is_not_ascii(&self) -> bool;
    /// The negation of [`eq_ignore_ascii_case`](std::ffi::OsStr::eq_ignore_ascii_case)
    #[must_use]
    fn not_eq_ignore_ascii_case<S>(&self, other: S) -> bool where S: AsRef<std::ffi::OsStr>;
}

impl os_str_private::Sealed for std::ffi::OsStr { }

impl IsntOsStrExt for std::ffi::OsStr {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn is_not_ascii(&self) -> bool {
        !self.is_ascii()
    }

    #[inline]
    fn not_eq_ignore_ascii_case<S>(&self, other: S) -> bool where S: AsRef<std::ffi::OsStr> {
        !self.eq_ignore_ascii_case::<S>(other)
    }
}
