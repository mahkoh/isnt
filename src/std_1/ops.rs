// This file was generated

mod range_private { pub trait Sealed<Idx> { } }

/// Extension for [`Range`](std::ops::Range)
pub trait IsntRangeExt<Idx>: range_private::Sealed<Idx> {
    /// The negation of [`contains`](std::ops::Range::contains)
    #[must_use]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized,;
    /// The negation of [`is_empty`](std::ops::Range::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<Idx> range_private::Sealed<Idx> for std::ops::Range<Idx> where Idx: PartialOrd<Idx>, { }

impl<Idx> IsntRangeExt<Idx> for std::ops::Range<Idx> where Idx: PartialOrd<Idx>, {
    #[inline]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized, {
        !self.contains::<U>(item)
    }

    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

mod range_from_private { pub trait Sealed<Idx> { } }

/// Extension for [`RangeFrom`](std::ops::RangeFrom)
pub trait IsntRangeFromExt<Idx>: range_from_private::Sealed<Idx> {
    /// The negation of [`contains`](std::ops::RangeFrom::contains)
    #[must_use]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized,;
}

impl<Idx> range_from_private::Sealed<Idx> for std::ops::RangeFrom<Idx> where Idx: PartialOrd<Idx>, { }

impl<Idx> IsntRangeFromExt<Idx> for std::ops::RangeFrom<Idx> where Idx: PartialOrd<Idx>, {
    #[inline]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized, {
        !self.contains::<U>(item)
    }
}

mod range_inclusive_private { pub trait Sealed<Idx> { } }

/// Extension for [`RangeInclusive`](std::ops::RangeInclusive)
pub trait IsntRangeInclusiveExt<Idx>: range_inclusive_private::Sealed<Idx> {
    /// The negation of [`contains`](std::ops::RangeInclusive::contains)
    #[must_use]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized,;
    /// The negation of [`is_empty`](std::ops::RangeInclusive::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<Idx> range_inclusive_private::Sealed<Idx> for std::ops::RangeInclusive<Idx> where Idx: PartialOrd<Idx>, { }

impl<Idx> IsntRangeInclusiveExt<Idx> for std::ops::RangeInclusive<Idx> where Idx: PartialOrd<Idx>, {
    #[inline]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized, {
        !self.contains::<U>(item)
    }

    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

mod range_to_private { pub trait Sealed<Idx> { } }

/// Extension for [`RangeTo`](std::ops::RangeTo)
pub trait IsntRangeToExt<Idx>: range_to_private::Sealed<Idx> {
    /// The negation of [`contains`](std::ops::RangeTo::contains)
    #[must_use]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized,;
}

impl<Idx> range_to_private::Sealed<Idx> for std::ops::RangeTo<Idx> where Idx: PartialOrd<Idx>, { }

impl<Idx> IsntRangeToExt<Idx> for std::ops::RangeTo<Idx> where Idx: PartialOrd<Idx>, {
    #[inline]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized, {
        !self.contains::<U>(item)
    }
}

mod range_to_inclusive_private { pub trait Sealed<Idx> { } }

/// Extension for [`RangeToInclusive`](std::ops::RangeToInclusive)
pub trait IsntRangeToInclusiveExt<Idx>: range_to_inclusive_private::Sealed<Idx> {
    /// The negation of [`contains`](std::ops::RangeToInclusive::contains)
    #[must_use]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized,;
}

impl<Idx> range_to_inclusive_private::Sealed<Idx> for std::ops::RangeToInclusive<Idx> where Idx: PartialOrd<Idx>, { }

impl<Idx> IsntRangeToInclusiveExt<Idx> for std::ops::RangeToInclusive<Idx> where Idx: PartialOrd<Idx>, {
    #[inline]
    fn not_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: PartialOrd<Idx> + ?Sized, {
        !self.contains::<U>(item)
    }
}
