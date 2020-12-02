// This file was generated

mod poll_private { pub trait Sealed<T> { } }

/// Extension for [`Poll`](std::task::Poll)
pub trait IsntPollExt<T>: poll_private::Sealed<T> {
    /// The negation of [`is_ready`](std::task::Poll::is_ready)
    #[must_use]
    fn is_not_ready(&self) -> bool;
    /// The negation of [`is_pending`](std::task::Poll::is_pending)
    #[must_use]
    fn is_not_pending(&self) -> bool;
}

impl<T> poll_private::Sealed<T> for std::task::Poll<T> { }

impl<T> IsntPollExt<T> for std::task::Poll<T> {
    #[inline]
    fn is_not_ready(&self) -> bool {
        !self.is_ready()
    }

    #[inline]
    fn is_not_pending(&self) -> bool {
        !self.is_pending()
    }
}

mod waker_private { pub trait Sealed { } }

/// Extension for [`Waker`](std::task::Waker)
pub trait IsntWakerExt: waker_private::Sealed {
    /// The negation of [`will_wake`](std::task::Waker::will_wake)
    #[must_use]
    fn not_will_wake(&self, other: &std::task::Waker) -> bool;
}

impl waker_private::Sealed for std::task::Waker { }

impl IsntWakerExt for std::task::Waker {
    #[inline]
    fn not_will_wake(&self, other: &std::task::Waker) -> bool {
        !self.will_wake(other)
    }
}
