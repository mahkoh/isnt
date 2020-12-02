// This file was generated

mod iterator_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for [`Iterator`](std::iter::Iterator)
pub trait IsntIteratorExt<T: ?Sized>: iterator_private::Sealed<T>+std::iter::Iterator {
    /// The negation of [`all`](std::iter::Iterator::all)
    #[must_use]
    fn not_all<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool, Self: Sized,;
    /// The negation of [`any`](std::iter::Iterator::any)
    #[must_use]
    fn not_any<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool, Self: Sized,;
    /// The negation of [`eq`](std::iter::Iterator::eq)
    #[must_use]
    fn not_eq<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialEq<<I as std::iter::IntoIterator>::Item>, Self: Sized;
    /// The negation of [`ne`](std::iter::Iterator::ne)
    #[must_use]
    fn not_ne<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialEq<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`lt`](std::iter::Iterator::lt)
    #[must_use]
    fn not_lt<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialOrd<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`le`](std::iter::Iterator::le)
    #[must_use]
    fn not_le<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialOrd<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`gt`](std::iter::Iterator::gt)
    #[must_use]
    fn not_gt<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialOrd<<I as IntoIterator>::Item>, Self: Sized;
    /// The negation of [`ge`](std::iter::Iterator::ge)
    #[must_use]
    fn not_ge<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialOrd<<I as IntoIterator>::Item>, Self: Sized;
}

impl<T: ?Sized> iterator_private::Sealed<T> for T where T: std::iter::Iterator { }

impl<T: ?Sized> IsntIteratorExt<T> for T where T: std::iter::Iterator {
    #[inline]
    fn not_all<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool, Self: Sized, {
        !self.all::<F>(f)
    }

    #[inline]
    fn not_any<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool, Self: Sized, {
        !self.any::<F>(f)
    }

    #[inline]
    fn not_eq<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialEq<<I as std::iter::IntoIterator>::Item>, Self: Sized {
        !self.eq::<I>(other)
    }

    #[inline]
    fn not_ne<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialEq<<I as IntoIterator>::Item>, Self: Sized {
        !self.ne::<I>(other)
    }

    #[inline]
    fn not_lt<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialOrd<<I as IntoIterator>::Item>, Self: Sized {
        !self.lt::<I>(other)
    }

    #[inline]
    fn not_le<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialOrd<<I as IntoIterator>::Item>, Self: Sized {
        !self.le::<I>(other)
    }

    #[inline]
    fn not_gt<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialOrd<<I as IntoIterator>::Item>, Self: Sized {
        !self.gt::<I>(other)
    }

    #[inline]
    fn not_ge<I>(self, other: I) -> bool where I: std::iter::IntoIterator, Self::Item: std::cmp::PartialOrd<<I as IntoIterator>::Item>, Self: Sized {
        !self.ge::<I>(other)
    }
}
