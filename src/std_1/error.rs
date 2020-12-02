// This file was generated

mod error_private { pub trait Sealed { } }

/// Extension for [`dyn`](dyn)
pub trait IsntErrorExt: error_private::Sealed {
    /// The negation of [`is`](dyn::is)
    #[must_use]
    fn is_not<T: std::error::Error + 'static>(&self) -> bool;
}

impl error_private::Sealed for dyn std::error::Error + 'static { }

impl IsntErrorExt for dyn std::error::Error + 'static {
    #[inline]
    fn is_not<T: std::error::Error + 'static>(&self) -> bool {
        !self.is::<T>()
    }
}

mod error2_private { pub trait Sealed { } }

/// Extension for [`dyn`](dyn)
pub trait IsntError2Ext: error2_private::Sealed {
    /// The negation of [`is`](dyn::is)
    #[must_use]
    fn is_not<T: std::error::Error + 'static>(&self) -> bool;
}

impl error2_private::Sealed for dyn std::error::Error + 'static + Send { }

impl IsntError2Ext for dyn std::error::Error + 'static + Send {
    #[inline]
    fn is_not<T: std::error::Error + 'static>(&self) -> bool {
        !self.is::<T>()
    }
}

mod error3_private { pub trait Sealed { } }

/// Extension for [`dyn`](dyn)
pub trait IsntError3Ext: error3_private::Sealed {
    /// The negation of [`is`](dyn::is)
    #[must_use]
    fn is_not<T: std::error::Error + 'static>(&self) -> bool;
}

impl error3_private::Sealed for dyn std::error::Error + 'static + Send + Sync { }

impl IsntError3Ext for dyn std::error::Error + 'static + Send + Sync {
    #[inline]
    fn is_not<T: std::error::Error + 'static>(&self) -> bool {
        !self.is::<T>()
    }
}
