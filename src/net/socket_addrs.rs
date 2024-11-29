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
    type Iter: Iterator<Item = SocketAddrV4>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError>;
}

impl ToSocketAddrs for SocketAddrV4 {
    type Iter = option::IntoIter<SocketAddrV4>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        Ok(Some(*self).into_iter())
    }
}

impl ToSocketAddrs for (Ipv4Addr, u16) {
    type Iter = option::IntoIter<SocketAddrV4>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        let (ip, port) = *self;
        Ok(Some(SocketAddrV4::new(ip, port)).into_iter())
    }
}

impl ToSocketAddrs for (&str, u16) {
    type Iter = vec::IntoIter<SocketAddrV4>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        let (host, port) = *self;
        match host.parse::<Ipv4Addr>() {
            Ok(addr) => Ok(vec![SocketAddrV4::new(addr, port)].into_iter()),
            Err(_) => Err(ToSocketAddrsError::CannotResolveHostname),
        }
    }
}

impl ToSocketAddrs for (String, u16) {
    type Iter = vec::IntoIter<SocketAddrV4>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        (&*self.0, self.1).to_socket_addrs()
    }
}

impl ToSocketAddrs for str {
    type Iter = vec::IntoIter<SocketAddrV4>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        match self.parse::<SocketAddrV4>() {
            Ok(addr) => Ok(vec![addr].into_iter()),
            Err(_) => Err(ToSocketAddrsError::CannotResolveHostname),
        }
    }
}

impl<'a> ToSocketAddrs for &'a [SocketAddrV4] {
    type Iter = iter::Cloned<slice::Iter<'a, SocketAddrV4>>;
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
    type Iter = vec::IntoIter<SocketAddrV4>;
    fn to_socket_addrs(&self) -> Result<Self::Iter, ToSocketAddrsError> {
        (&**self).to_socket_addrs()
    }
}
