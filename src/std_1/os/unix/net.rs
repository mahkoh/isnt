// This file was generated

#[cfg(unix)]
mod socket_addr_private { pub trait Sealed { } }

/// Extension for [`SocketAddr`](std::os::unix::net::SocketAddr)
#[cfg(unix)]
pub trait IsntSocketAddrExt: socket_addr_private::Sealed {
    /// The negation of [`is_unnamed`](std::os::unix::net::SocketAddr::is_unnamed)
    #[must_use]
    fn is_not_unnamed(&self) -> bool;
}

#[cfg(unix)]
impl socket_addr_private::Sealed for std::os::unix::net::SocketAddr { }

#[cfg(unix)]
impl IsntSocketAddrExt for std::os::unix::net::SocketAddr {
    #[inline]
    fn is_not_unnamed(&self) -> bool {
        !self.is_unnamed()
    }
}
