// This file was generated

mod is_terminal_private { pub trait Sealed<T> { } }

/// Extension for [`IsTerminal`](std::io::IsTerminal)
pub trait IsntIsTerminalExt<T>: is_terminal_private::Sealed<T>+std::io::IsTerminal {
    /// The negation of [`is_terminal`](std::io::IsTerminal::is_terminal)
    #[must_use]
    fn is_not_terminal(&self) -> bool;
}

impl<T> is_terminal_private::Sealed<T> for T where T: std::io::IsTerminal { }

impl<T> IsntIsTerminalExt<T> for T where T: std::io::IsTerminal {
    #[inline]
    fn is_not_terminal(&self) -> bool {
        !self.is_terminal()
    }
}
