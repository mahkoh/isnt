// This file was generated

#[cfg(unix)]
mod exit_status_ext_private { pub trait Sealed<T: ?Sized> { } }

/// Extension for [`ExitStatusExt`](std::os::unix::process::ExitStatusExt)
#[cfg(unix)]
pub trait IsntExitStatusExtExt<T: ?Sized>: exit_status_ext_private::Sealed<T>+std::os::unix::process::ExitStatusExt {
    /// The negation of [`core_dumped`](std::os::unix::process::ExitStatusExt::core_dumped)
    #[must_use]
    fn not_core_dumped(&self) -> bool;
    /// The negation of [`continued`](std::os::unix::process::ExitStatusExt::continued)
    #[must_use]
    fn not_continued(&self) -> bool;
}

#[cfg(unix)]
impl<T: ?Sized> exit_status_ext_private::Sealed<T> for T where T: std::os::unix::process::ExitStatusExt { }

#[cfg(unix)]
impl<T: ?Sized> IsntExitStatusExtExt<T> for T where T: std::os::unix::process::ExitStatusExt {
    #[inline]
    fn not_core_dumped(&self) -> bool {
        !self.core_dumped()
    }

    #[inline]
    fn not_continued(&self) -> bool {
        !self.continued()
    }
}
