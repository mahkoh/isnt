// This file was generated

mod join_handle_private { pub trait Sealed<T> { } }

/// Extension for [`JoinHandle`](std::thread::JoinHandle)
pub trait IsntJoinHandleExt<T>: join_handle_private::Sealed<T> {
    /// The negation of [`is_finished`](std::thread::JoinHandle::is_finished)
    #[must_use]
    fn is_not_finished(&self) -> bool;
}

impl<T> join_handle_private::Sealed<T> for std::thread::JoinHandle<T> { }

impl<T> IsntJoinHandleExt<T> for std::thread::JoinHandle<T> {
    #[inline]
    fn is_not_finished(&self) -> bool {
        !self.is_finished()
    }
}

mod scoped_join_handle_private { pub trait Sealed<'a, T> { } }

/// Extension for [`ScopedJoinHandle`](std::thread::ScopedJoinHandle)
pub trait IsntScopedJoinHandleExt<'a, T>: scoped_join_handle_private::Sealed<'a, T> {
    /// The negation of [`is_finished`](std::thread::ScopedJoinHandle::is_finished)
    #[must_use]
    fn is_not_finished(&self) -> bool;
}

impl<'a, T> scoped_join_handle_private::Sealed<'a, T> for std::thread::ScopedJoinHandle<'a, T> { }

impl<'a, T> IsntScopedJoinHandleExt<'a, T> for std::thread::ScopedJoinHandle<'a, T> {
    #[inline]
    fn is_not_finished(&self) -> bool {
        !self.is_finished()
    }
}
