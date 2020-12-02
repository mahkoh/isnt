// This file was generated

mod exit_status_private { pub trait Sealed { } }

/// Extension for [`ExitStatus`](std::process::ExitStatus)
pub trait IsntExitStatusExt: exit_status_private::Sealed {
    /// The negation of [`success`](std::process::ExitStatus::success)
    #[must_use]
    fn not_success(&self) -> bool;
}

impl exit_status_private::Sealed for std::process::ExitStatus { }

impl IsntExitStatusExt for std::process::ExitStatus {
    #[inline]
    fn not_success(&self) -> bool {
        !self.success()
    }
}
