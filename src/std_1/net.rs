// This file was generated

mod ip_addr_private { pub trait Sealed { } }

/// Extension for [`Ipv6Addr`](std::net::Ipv6Addr)
pub trait IsntIpAddrExt: ip_addr_private::Sealed {
    /// The negation of [`is_unspecified`](std::net::Ipv6Addr::is_unspecified)
    #[must_use]
    fn is_not_unspecified(&self) -> bool;
    /// The negation of [`is_loopback`](std::net::Ipv6Addr::is_loopback)
    #[must_use]
    fn is_not_loopback(&self) -> bool;
    /// The negation of [`is_multicast`](std::net::Ipv6Addr::is_multicast)
    #[must_use]
    fn is_not_multicast(&self) -> bool;
}

impl ip_addr_private::Sealed for std::net::Ipv6Addr { }

impl IsntIpAddrExt for std::net::Ipv6Addr {
    #[inline]
    fn is_not_unspecified(&self) -> bool {
        !self.is_unspecified()
    }

    #[inline]
    fn is_not_loopback(&self) -> bool {
        !self.is_loopback()
    }

    #[inline]
    fn is_not_multicast(&self) -> bool {
        !self.is_multicast()
    }
}

mod ipv4_addr_private { pub trait Sealed { } }

/// Extension for [`Ipv4Addr`](std::net::Ipv4Addr)
pub trait IsntIpv4AddrExt: ipv4_addr_private::Sealed {
    /// The negation of [`is_unspecified`](std::net::Ipv4Addr::is_unspecified)
    #[must_use]
    fn is_not_unspecified(&self) -> bool;
    /// The negation of [`is_loopback`](std::net::Ipv4Addr::is_loopback)
    #[must_use]
    fn is_not_loopback(&self) -> bool;
    /// The negation of [`is_private`](std::net::Ipv4Addr::is_private)
    #[must_use]
    fn is_not_private(&self) -> bool;
    /// The negation of [`is_link_local`](std::net::Ipv4Addr::is_link_local)
    #[must_use]
    fn is_not_link_local(&self) -> bool;
    /// The negation of [`is_multicast`](std::net::Ipv4Addr::is_multicast)
    #[must_use]
    fn is_not_multicast(&self) -> bool;
    /// The negation of [`is_broadcast`](std::net::Ipv4Addr::is_broadcast)
    #[must_use]
    fn is_not_broadcast(&self) -> bool;
    /// The negation of [`is_documentation`](std::net::Ipv4Addr::is_documentation)
    #[must_use]
    fn is_not_documentation(&self) -> bool;
}

impl ipv4_addr_private::Sealed for std::net::Ipv4Addr { }

impl IsntIpv4AddrExt for std::net::Ipv4Addr {
    #[inline]
    fn is_not_unspecified(&self) -> bool {
        !self.is_unspecified()
    }

    #[inline]
    fn is_not_loopback(&self) -> bool {
        !self.is_loopback()
    }

    #[inline]
    fn is_not_private(&self) -> bool {
        !self.is_private()
    }

    #[inline]
    fn is_not_link_local(&self) -> bool {
        !self.is_link_local()
    }

    #[inline]
    fn is_not_multicast(&self) -> bool {
        !self.is_multicast()
    }

    #[inline]
    fn is_not_broadcast(&self) -> bool {
        !self.is_broadcast()
    }

    #[inline]
    fn is_not_documentation(&self) -> bool {
        !self.is_documentation()
    }
}

mod ipv6_addr_private { pub trait Sealed { } }

/// Extension for [`IpAddr`](std::net::IpAddr)
pub trait IsntIpv6AddrExt: ipv6_addr_private::Sealed {
    /// The negation of [`is_unspecified`](std::net::IpAddr::is_unspecified)
    #[must_use]
    fn is_not_unspecified(&self) -> bool;
    /// The negation of [`is_loopback`](std::net::IpAddr::is_loopback)
    #[must_use]
    fn is_not_loopback(&self) -> bool;
    /// The negation of [`is_multicast`](std::net::IpAddr::is_multicast)
    #[must_use]
    fn is_not_multicast(&self) -> bool;
    /// The negation of [`is_ipv4`](std::net::IpAddr::is_ipv4)
    #[must_use]
    fn is_not_ipv4(&self) -> bool;
    /// The negation of [`is_ipv6`](std::net::IpAddr::is_ipv6)
    #[must_use]
    fn is_not_ipv6(&self) -> bool;
}

impl ipv6_addr_private::Sealed for std::net::IpAddr { }

impl IsntIpv6AddrExt for std::net::IpAddr {
    #[inline]
    fn is_not_unspecified(&self) -> bool {
        !self.is_unspecified()
    }

    #[inline]
    fn is_not_loopback(&self) -> bool {
        !self.is_loopback()
    }

    #[inline]
    fn is_not_multicast(&self) -> bool {
        !self.is_multicast()
    }

    #[inline]
    fn is_not_ipv4(&self) -> bool {
        !self.is_ipv4()
    }

    #[inline]
    fn is_not_ipv6(&self) -> bool {
        !self.is_ipv6()
    }
}

mod socket_addr_private { pub trait Sealed { } }

/// Extension for [`SocketAddr`](std::net::SocketAddr)
pub trait IsntSocketAddrExt: socket_addr_private::Sealed {
    /// The negation of [`is_ipv4`](std::net::SocketAddr::is_ipv4)
    #[must_use]
    fn is_not_ipv4(&self) -> bool;
    /// The negation of [`is_ipv6`](std::net::SocketAddr::is_ipv6)
    #[must_use]
    fn is_not_ipv6(&self) -> bool;
}

impl socket_addr_private::Sealed for std::net::SocketAddr { }

impl IsntSocketAddrExt for std::net::SocketAddr {
    #[inline]
    fn is_not_ipv4(&self) -> bool {
        !self.is_ipv4()
    }

    #[inline]
    fn is_not_ipv6(&self) -> bool {
        !self.is_ipv6()
    }
}
