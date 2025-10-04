// This file was generated

mod ordering_private { pub trait Sealed { } }

/// Extension for [`Ordering`](std::cmp::Ordering)
pub trait IsntOrderingExt: ordering_private::Sealed {
    /// The negation of [`is_eq`](std::cmp::Ordering::is_eq)
    #[must_use]
    fn is_not_eq(self) -> bool;
    /// The negation of [`is_ge`](std::cmp::Ordering::is_ge)
    #[must_use]
    fn is_not_ge(self) -> bool;
    /// The negation of [`is_gt`](std::cmp::Ordering::is_gt)
    #[must_use]
    fn is_not_gt(self) -> bool;
    /// The negation of [`is_le`](std::cmp::Ordering::is_le)
    #[must_use]
    fn is_not_le(self) -> bool;
    /// The negation of [`is_lt`](std::cmp::Ordering::is_lt)
    #[must_use]
    fn is_not_lt(self) -> bool;
    /// The negation of [`is_ne`](std::cmp::Ordering::is_ne)
    #[must_use]
    fn is_not_ne(self) -> bool;
}

impl ordering_private::Sealed for std::cmp::Ordering { }

impl IsntOrderingExt for std::cmp::Ordering {
    #[inline]
    fn is_not_eq(self) -> bool {
        !self.is_eq()
    }

    #[inline]
    fn is_not_ge(self) -> bool {
        !self.is_ge()
    }

    #[inline]
    fn is_not_gt(self) -> bool {
        !self.is_gt()
    }

    #[inline]
    fn is_not_le(self) -> bool {
        !self.is_le()
    }

    #[inline]
    fn is_not_lt(self) -> bool {
        !self.is_lt()
    }

    #[inline]
    fn is_not_ne(self) -> bool {
        !self.is_ne()
    }
}

mod partial_eq_private { pub trait Sealed<Lhs: ?Sized, Rhs: ?Sized> { } }

/// Extension for [`PartialEq`](PartialEq)
pub trait IsntPartialEqExt<Lhs: ?Sized, Rhs: ?Sized>: partial_eq_private::Sealed<Lhs, Rhs>+PartialEq<Rhs> {
    /// The negation of [`eq`](PartialEq::eq)
    #[must_use]
    fn not_eq(&self, other: &Rhs) -> bool;
    /// The negation of [`ne`](PartialEq::ne)
    #[must_use]
    fn not_ne(&self, other: &Rhs) -> bool;
}

impl<Lhs: ?Sized, Rhs: ?Sized> partial_eq_private::Sealed<Lhs, Rhs> for Lhs where Lhs: PartialEq<Rhs> { }

impl<Lhs: ?Sized, Rhs: ?Sized> IsntPartialEqExt<Lhs, Rhs> for Lhs where Lhs: PartialEq<Rhs> {
    #[inline]
    fn not_eq(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }

    #[inline]
    fn not_ne(&self, other: &Rhs) -> bool {
        !self.ne(other)
    }
}

mod partial_ord_private { pub trait Sealed<Lhs: ?Sized, Rhs: ?Sized> { } }

/// Extension for [`PartialOrd`](PartialOrd)
pub trait IsntPartialOrdExt<Lhs: ?Sized, Rhs: ?Sized>: partial_ord_private::Sealed<Lhs, Rhs>+PartialOrd<Rhs> {
    /// The negation of [`lt`](PartialOrd::lt)
    #[must_use]
    fn not_lt(&self, other: &Rhs) -> bool;
    /// The negation of [`le`](PartialOrd::le)
    #[must_use]
    fn not_le(&self, other: &Rhs) -> bool;
    /// The negation of [`gt`](PartialOrd::gt)
    #[must_use]
    fn not_gt(&self, other: &Rhs) -> bool;
    /// The negation of [`ge`](PartialOrd::ge)
    #[must_use]
    fn not_ge(&self, other: &Rhs) -> bool;
}

impl<Lhs: ?Sized, Rhs: ?Sized> partial_ord_private::Sealed<Lhs, Rhs> for Lhs where Lhs: PartialOrd<Rhs> { }

impl<Lhs: ?Sized, Rhs: ?Sized> IsntPartialOrdExt<Lhs, Rhs> for Lhs where Lhs: PartialOrd<Rhs> {
    #[inline]
    fn not_lt(&self, other: &Rhs) -> bool {
        !self.lt(other)
    }

    #[inline]
    fn not_le(&self, other: &Rhs) -> bool {
        !self.le(other)
    }

    #[inline]
    fn not_gt(&self, other: &Rhs) -> bool {
        !self.gt(other)
    }

    #[inline]
    fn not_ge(&self, other: &Rhs) -> bool {
        !self.ge(other)
    }
}
