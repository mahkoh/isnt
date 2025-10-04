// This file was generated

mod iterator_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for [`Iterator`](Iterator)
pub trait IsntIteratorExt<T: ?Sized>: iterator_private::Sealed<T>+Iterator {
    /// The negation of [`all`](Iterator::all)
    #[must_use]
    fn not_all<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool, Self: Sized,;
    /// The negation of [`any`](Iterator::any)
    #[must_use]
    fn not_any<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool, Self: Sized,;
    /// The negation of [`eq`](Iterator::eq)
    #[must_use]
    fn not_eq<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`ne`](Iterator::ne)
    #[must_use]
    fn not_ne<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`lt`](Iterator::lt)
    #[must_use]
    fn not_lt<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`le`](Iterator::le)
    #[must_use]
    fn not_le<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`gt`](Iterator::gt)
    #[must_use]
    fn not_gt<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`ge`](Iterator::ge)
    #[must_use]
    fn not_ge<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`is_sorted`](Iterator::is_sorted)
    #[must_use]
    fn is_not_sorted(self) -> bool where Self: Sized, Self::Item: PartialOrd;
    /// The negation of [`is_sorted_by`](Iterator::is_sorted_by)
    #[must_use]
    fn is_not_sorted_by<F>(self, compare: F) -> bool where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> bool;
    /// The negation of [`is_sorted_by_key`](Iterator::is_sorted_by_key)
    #[must_use]
    fn is_not_sorted_by_key<F, K>(self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> K, K: PartialOrd;
}

impl<T: ?Sized> iterator_private::Sealed<T> for T where T: Iterator { }

impl<T: ?Sized> IsntIteratorExt<T> for T where T: Iterator {
    #[inline]
    fn not_all<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool, Self: Sized, {
        !self.all::<F>(f)
    }

    #[inline]
    fn not_any<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool, Self: Sized, {
        !self.any::<F>(f)
    }

    #[inline]
    fn not_eq<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<<I as IntoIterator>::Item>, Self: Sized {
        !self.eq::<I>(other)
    }

    #[inline]
    fn not_ne<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<<I as IntoIterator>::Item>, Self: Sized {
        !self.ne::<I>(other)
    }

    #[inline]
    fn not_lt<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized {
        !self.lt::<I>(other)
    }

    #[inline]
    fn not_le<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized {
        !self.le::<I>(other)
    }

    #[inline]
    fn not_gt<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized {
        !self.gt::<I>(other)
    }

    #[inline]
    fn not_ge<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized {
        !self.ge::<I>(other)
    }

    #[inline]
    fn is_not_sorted(self) -> bool where Self: Sized, Self::Item: PartialOrd {
        !self.is_sorted()
    }

    #[inline]
    fn is_not_sorted_by<F>(self, compare: F) -> bool where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> bool {
        !self.is_sorted_by::<F>(compare)
    }

    #[inline]
    fn is_not_sorted_by_key<F, K>(self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> K, K: PartialOrd {
        !self.is_sorted_by_key::<F, K>(f)
    }
}
