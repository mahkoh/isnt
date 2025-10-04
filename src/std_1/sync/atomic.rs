// This file was generated

mod atomic_bool_private { pub trait Sealed { } }

/// Extension for [`AtomicBool`](std::sync::atomic::AtomicBool)
pub trait IsntAtomicBoolExt: atomic_bool_private::Sealed {
    /// The negation of [`into_inner`](std::sync::atomic::AtomicBool::into_inner)
    #[must_use]
    fn not_into_inner(self) -> bool;
    /// The negation of [`load`](std::sync::atomic::AtomicBool::load)
    #[must_use]
    fn not_load(&self, order: std::sync::atomic::Ordering) -> bool;
    /// The negation of [`swap`](std::sync::atomic::AtomicBool::swap)
    #[must_use]
    fn not_swap(&self, val: bool, order: std::sync::atomic::Ordering) -> bool;
    /// The negation of [`fetch_and`](std::sync::atomic::AtomicBool::fetch_and)
    #[must_use]
    fn not_fetch_and(&self, val: bool, order: std::sync::atomic::Ordering) -> bool;
    /// The negation of [`fetch_nand`](std::sync::atomic::AtomicBool::fetch_nand)
    #[must_use]
    fn not_fetch_nand(&self, val: bool, order: std::sync::atomic::Ordering) -> bool;
    /// The negation of [`fetch_xor`](std::sync::atomic::AtomicBool::fetch_xor)
    #[must_use]
    fn not_fetch_xor(&self, val: bool, order: std::sync::atomic::Ordering) -> bool;
    /// The negation of [`fetch_not`](std::sync::atomic::AtomicBool::fetch_not)
    #[must_use]
    fn not_fetch_not(&self, order: std::sync::atomic::Ordering) -> bool;
}

impl atomic_bool_private::Sealed for std::sync::atomic::AtomicBool { }

impl IsntAtomicBoolExt for std::sync::atomic::AtomicBool {
    #[inline]
    fn not_into_inner(self) -> bool {
        !self.into_inner()
    }

    #[inline]
    fn not_load(&self, order: std::sync::atomic::Ordering) -> bool {
        !self.load(order)
    }

    #[inline]
    fn not_swap(&self, val: bool, order: std::sync::atomic::Ordering) -> bool {
        !self.swap(val, order)
    }

    #[inline]
    fn not_fetch_and(&self, val: bool, order: std::sync::atomic::Ordering) -> bool {
        !self.fetch_and(val, order)
    }

    #[inline]
    fn not_fetch_nand(&self, val: bool, order: std::sync::atomic::Ordering) -> bool {
        !self.fetch_nand(val, order)
    }

    #[inline]
    fn not_fetch_xor(&self, val: bool, order: std::sync::atomic::Ordering) -> bool {
        !self.fetch_xor(val, order)
    }

    #[inline]
    fn not_fetch_not(&self, order: std::sync::atomic::Ordering) -> bool {
        !self.fetch_not(order)
    }
}
