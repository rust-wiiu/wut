// socket_addrs

//! Basically copied from https://doc.rust-lang.org/src/std/net/socket_addr.rs.html
//! Conversion from common representations to address

use alloc::{string::String, vec};
use core::{iter, net::*, option, slice};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToSocketAddrsError {
    #[error("failure when parsing address")]
    AddressParse(#[from] AddrParseError),
    #[error("failure when resolving address")]
    CannotResolveHostname,
}

pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError>;
}

impl ToSocketAddrs for SocketAddr {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        Ok(Some(*self).into_iter())
    }
}

impl ToSocketAddrs for SocketAddrV4 {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        SocketAddr::V4(*self).to_socket_addrs()
    }
}

impl ToSocketAddrs for SocketAddrV6 {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        SocketAddr::V6(*self).to_socket_addrs()
    }
}

impl ToSocketAddrs for (IpAddr, u16) {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        let (ip, port) = *self;
        match ip {
            IpAddr::V4(ref a) => (*a, port).to_socket_addrs(),
            IpAddr::V6(ref a) => (*a, port).to_socket_addrs(),
        }
    }
}

impl ToSocketAddrs for (Ipv4Addr, u16) {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        let (ip, port) = *self;
        SocketAddrV4::new(ip, port).to_socket_addrs()
    }
}

impl ToSocketAddrs for (Ipv6Addr, u16) {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        let (ip, port) = *self;
        SocketAddrV6::new(ip, port, 0, 0).to_socket_addrs()
    }
}

impl ToSocketAddrs for (&str, u16) {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        let (host, port) = *self;

        if let Ok(addr) = host.parse::<Ipv4Addr>() {
            let addr = SocketAddrV4::new(addr, port);
            Ok(vec![SocketAddr::V4(addr)].into_iter())
        } else if let Ok(addr) = host.parse::<Ipv6Addr>() {
            let addr = SocketAddrV6::new(addr, port, 0, 0);
            Ok(vec![SocketAddr::V6(addr)].into_iter())
        } else {
            Err(ToSocketAddrsError::CannotResolveHostname)
        }
    }
}

impl ToSocketAddrs for (String, u16) {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        (&*self.0, self.1).to_socket_addrs()
    }
}

impl ToSocketAddrs for str {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        if let Ok(addr) = self.parse::<SocketAddr>() {
            Ok(vec![addr].into_iter())
        } else {
            Err(ToSocketAddrsError::CannotResolveHostname)
        }
    }
}

impl<'a> ToSocketAddrs for &'a [SocketAddr] {
    type Iter = iter::Cloned<slice::Iter<'a, SocketAddr>>;

    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        Ok(self.iter().cloned())
    }
}

impl<T: ToSocketAddrs + ?Sized> ToSocketAddrs for &T {
    type Iter = T::Iter;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        (**self).to_socket_addrs()
    }
}

impl ToSocketAddrs for String {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        (&**self).to_socket_addrs()
    }
}
