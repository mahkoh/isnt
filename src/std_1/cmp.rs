// This file was generated

mod partial_eq_private { pub trait Sealed<Lhs: ?Sized, Rhs: ?Sized> { } }

/// Extension for [`PartialEq`](std::cmp::PartialEq)
pub trait IsntPartialEqExt<Lhs: ?Sized, Rhs: ?Sized>: partial_eq_private::Sealed<Lhs, Rhs>+std::cmp::PartialEq<Rhs> {
    /// The negation of [`eq`](std::cmp::PartialEq::eq)
    #[must_use]
    fn not_eq(&self, other: &Rhs) -> bool;
    /// The negation of [`ne`](std::cmp::PartialEq::ne)
    #[must_use]
    fn not_ne(&self, other: &Rhs) -> bool;
}

impl<Lhs: ?Sized, Rhs: ?Sized> partial_eq_private::Sealed<Lhs, Rhs> for Lhs where Lhs: std::cmp::PartialEq<Rhs> { }

impl<Lhs: ?Sized, Rhs: ?Sized> IsntPartialEqExt<Lhs, Rhs> for Lhs where Lhs: std::cmp::PartialEq<Rhs> {
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

/// Extension for [`PartialOrd`](std::cmp::PartialOrd)
pub trait IsntPartialOrdExt<Lhs: ?Sized, Rhs: ?Sized>: partial_ord_private::Sealed<Lhs, Rhs>+std::cmp::PartialOrd<Rhs> {
    /// The negation of [`lt`](std::cmp::PartialOrd::lt)
    #[must_use]
    fn not_lt(&self, other: &Rhs) -> bool;
    /// The negation of [`le`](std::cmp::PartialOrd::le)
    #[must_use]
    fn not_le(&self, other: &Rhs) -> bool;
    /// The negation of [`gt`](std::cmp::PartialOrd::gt)
    #[must_use]
    fn not_gt(&self, other: &Rhs) -> bool;
    /// The negation of [`ge`](std::cmp::PartialOrd::ge)
    #[must_use]
    fn not_ge(&self, other: &Rhs) -> bool;
}

impl<Lhs: ?Sized, Rhs: ?Sized> partial_ord_private::Sealed<Lhs, Rhs> for Lhs where Lhs: std::cmp::PartialOrd<Rhs> { }

impl<Lhs: ?Sized, Rhs: ?Sized> IsntPartialOrdExt<Lhs, Rhs> for Lhs where Lhs: std::cmp::PartialOrd<Rhs> {
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
