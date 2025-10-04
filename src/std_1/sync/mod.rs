// This file was generated

pub mod atomic;
mod barrier_wait_result_private { pub trait Sealed { } }

/// Extension for [`BarrierWaitResult`](std::sync::BarrierWaitResult)
pub trait IsntBarrierWaitResultExt: barrier_wait_result_private::Sealed {
    /// The negation of [`is_leader`](std::sync::BarrierWaitResult::is_leader)
    #[must_use]
    fn is_not_leader(&self) -> bool;
}

impl barrier_wait_result_private::Sealed for std::sync::BarrierWaitResult { }

impl IsntBarrierWaitResultExt for std::sync::BarrierWaitResult {
    #[inline]
    fn is_not_leader(&self) -> bool {
        !self.is_leader()
    }
}

mod mutex_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for [`Mutex`](std::sync::Mutex)
pub trait IsntMutexExt<T: ?Sized>: mutex_private::Sealed<T> {
    /// The negation of [`is_poisoned`](std::sync::Mutex::is_poisoned)
    #[must_use]
    fn is_not_poisoned(&self) -> bool;
}

impl<T: ?Sized> mutex_private::Sealed<T> for std::sync::Mutex<T> { }

impl<T: ?Sized> IsntMutexExt<T> for std::sync::Mutex<T> {
    #[inline]
    fn is_not_poisoned(&self) -> bool {
        !self.is_poisoned()
    }
}

mod once_private { pub trait Sealed { } }

/// Extension for [`Once`](std::sync::Once)
pub trait IsntOnceExt: once_private::Sealed {
    /// The negation of [`is_completed`](std::sync::Once::is_completed)
    #[must_use]
    fn is_not_completed(&self) -> bool;
}

impl once_private::Sealed for std::sync::Once { }

impl IsntOnceExt for std::sync::Once {
    #[inline]
    fn is_not_completed(&self) -> bool {
        !self.is_completed()
    }
}

mod once_state_private { pub trait Sealed { } }

/// Extension for [`OnceState`](std::sync::OnceState)
pub trait IsntOnceStateExt: once_state_private::Sealed {
    /// The negation of [`is_poisoned`](std::sync::OnceState::is_poisoned)
    #[must_use]
    fn is_not_poisoned(&self) -> bool;
}

impl once_state_private::Sealed for std::sync::OnceState { }

impl IsntOnceStateExt for std::sync::OnceState {
    #[inline]
    fn is_not_poisoned(&self) -> bool {
        !self.is_poisoned()
    }
}

mod rw_lock_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for [`RwLock`](std::sync::RwLock)
pub trait IsntRwLockExt<T: ?Sized>: rw_lock_private::Sealed<T> {
    /// The negation of [`is_poisoned`](std::sync::RwLock::is_poisoned)
    #[must_use]
    fn is_not_poisoned(&self) -> bool;
}

impl<T: ?Sized> rw_lock_private::Sealed<T> for std::sync::RwLock<T> { }

impl<T: ?Sized> IsntRwLockExt<T> for std::sync::RwLock<T> {
    #[inline]
    fn is_not_poisoned(&self) -> bool {
        !self.is_poisoned()
    }
}

mod wait_timeout_result_private { pub trait Sealed { } }

/// Extension for [`WaitTimeoutResult`](std::sync::WaitTimeoutResult)
pub trait IsntWaitTimeoutResultExt: wait_timeout_result_private::Sealed {
    /// The negation of [`timed_out`](std::sync::WaitTimeoutResult::timed_out)
    #[must_use]
    fn not_timed_out(&self) -> bool;
}

impl wait_timeout_result_private::Sealed for std::sync::WaitTimeoutResult { }

impl IsntWaitTimeoutResultExt for std::sync::WaitTimeoutResult {
    #[inline]
    fn not_timed_out(&self) -> bool {
        !self.timed_out()
    }
}

mod weak_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for [`Weak`](std::sync::Weak)
pub trait IsntWeakExt<T: ?Sized>: weak_private::Sealed<T> {
    /// The negation of [`ptr_eq`](std::sync::Weak::ptr_eq)
    #[must_use]
    fn not_ptr_eq(&self, other: &std::sync::Weak<T>) -> bool;
}

impl<T: ?Sized> weak_private::Sealed<T> for std::sync::Weak<T> { }

impl<T: ?Sized> IsntWeakExt<T> for std::sync::Weak<T> {
    #[inline]
    fn not_ptr_eq(&self, other: &std::sync::Weak<T>) -> bool {
        !self.ptr_eq(other)
    }
}
