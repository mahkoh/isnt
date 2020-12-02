// This file was generated

mod any_private { pub trait Sealed { } }

/// Extension for [`dyn`](dyn)
pub trait IsntAnyExt: any_private::Sealed {
    /// The negation of [`is`](dyn::is)
    #[must_use]
    fn is_not<T: std::any::Any>(&self) -> bool;
}

impl any_private::Sealed for dyn std::any::Any + 'static { }

impl IsntAnyExt for dyn std::any::Any + 'static {
    #[inline]
    fn is_not<T: std::any::Any>(&self) -> bool {
        !self.is::<T>()
    }
}

mod any2_private { pub trait Sealed { } }

/// Extension for [`dyn`](dyn)
pub trait IsntAny2Ext: any2_private::Sealed {
    /// The negation of [`is`](dyn::is)
    #[must_use]
    fn is_not<T: std::any::Any>(&self) -> bool;
}

impl any2_private::Sealed for dyn std::any::Any + 'static + Send { }

impl IsntAny2Ext for dyn std::any::Any + 'static + Send {
    #[inline]
    fn is_not<T: std::any::Any>(&self) -> bool {
        !self.is::<T>()
    }
}

mod any3_private { pub trait Sealed { } }

/// Extension for [`dyn`](dyn)
pub trait IsntAny3Ext: any3_private::Sealed {
    /// The negation of [`is`](dyn::is)
    #[must_use]
    fn is_not<T: std::any::Any>(&self) -> bool;
}

impl any3_private::Sealed for dyn std::any::Any + 'static + Send + Sync { }

impl IsntAny3Ext for dyn std::any::Any + 'static + Send + Sync {
    #[inline]
    fn is_not<T: std::any::Any>(&self) -> bool {
        !self.is::<T>()
    }
}
