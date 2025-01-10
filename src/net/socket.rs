// socket

use crate::{
    bindings as c_wut,
    net::socket_addrs::{ToSocketAddrs, ToSocketAddrsError},
};
use core::net::{Ipv4Addr, SocketAddrV4};
use thiserror::Error;

#[derive(Debug)]
pub struct Socket(i32);

#[derive(Debug, Error)]
pub enum SocketError {
    #[error("failed to create TCP socket")]
    TcpCreation,
    #[error("failed to create UDP socket")]
    UdpCreation,
    #[error("invalid address provided")]
    InvalidAddress(#[from] ToSocketAddrsError),
    #[error("failed to bind to address")]
    NoAvailableAddress,
    #[error("failed to listen on socket")]
    CannotListenOnSocket,
    #[error("Connection closed by either of the parties")]
    ConnectionClosed,
    #[error("failed to accept incoming request")]
    CannotAccept,
    #[error("tmp - remove later on")]
    Errno(i32),
}

// impl TryFrom<i32> for SocketError {
//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         let errno = Self::errno();

//         if value > 0 {
//             return Ok(Self::Errno(value));
//         }

//         match  {

//         }
//     }
// }

impl SocketError {
    fn errno() -> i32 {
        unsafe { *c_wut::__errno() }
    }

    fn from_errno() -> Self {
        match Self::errno() {
            v => Self::Errno(v),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Shutdown {
    Read,
    Write,
    Both,
}

impl Socket {
    pub fn tcp() -> Result<Self, SocketError> {
        unsafe {
            let fd = c_wut::socket(
                c_wut::AF_INET as i32,
                c_wut::SOCK_STREAM as i32,
                c_wut::IPPROTO_TCP as i32,
            );

            if fd <= 0 {
                // c_wut::__errno()
                Err(SocketError::TcpCreation)
            } else {
                Ok(Self(fd))
            }
        }
    }

    pub fn udp() -> Result<Self, SocketError> {
        unsafe {
            let fd = c_wut::socket(
                c_wut::AF_INET as i32,
                c_wut::SOCK_DGRAM as i32,
                c_wut::IPPROTO_UDP as i32,
            );
            if fd <= 0 {
                Err(SocketError::UdpCreation)
            } else {
                Ok(Self(fd))
            }
        }
    }

    pub fn valid(&self) -> bool {
        self.0 > 0
    }

    pub fn bind(&self, addr: impl ToSocketAddrs) -> Result<SocketAddrV4, SocketError> {
        for address in addr.to_socket_addrs()? {
            let mut addr = c_wut::sockaddr_in::default();
            addr.sin_family = c_wut::AF_INET as u16;
            addr.sin_addr.s_addr = address.ip().to_bits();
            addr.sin_port = address.port();

            if unsafe { c_wut::bind(self.0, &addr as *const _ as *const c_wut::sockaddr, 16) } == 0
            {
                return Ok(address);
            }
        }

        Err(SocketError::NoAvailableAddress)
    }

    pub fn listen(&self, backlog: u32) -> Result<(), SocketError> {
        if unsafe { c_wut::listen(self.0, backlog as i32) } == 0 {
            Ok(())
        } else {
            Err(SocketError::CannotListenOnSocket)
        }
    }

    pub fn accept(&self) -> Result<(Socket, SocketAddrV4), SocketError> {
        let mut addr = c_wut::sockaddr_in::default();
        let mut len = size_of::<c_wut::sockaddr_in>() as u32;

        crate::println!("errno: {}", SocketError::errno());

        let fd = unsafe {
            c_wut::accept(
                self.0,
                &mut addr as *mut _ as *mut c_wut::sockaddr,
                &mut len,
            )
        };

        crate::println!("errno: {}", SocketError::errno());

        if fd < 0 {
            Err(SocketError::Errno(SocketError::errno()))
        } else if fd == 0 {
            Err(SocketError::CannotAccept)
        } else {
            let ip = Ipv4Addr::from(u32::from_be(addr.sin_addr.s_addr));
            let port = u16::from_be(addr.sin_port);
            let socket_addr = SocketAddrV4::new(ip, port);

            Ok((Socket(fd), socket_addr))
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, SocketError> {
        let bytes = unsafe { c_wut::recv(self.0, buf.as_mut_ptr() as *mut _, buf.len(), 0) };

        if bytes < 0 {
            Err(SocketError::from_errno())
        } else if bytes == 0 {
            Err(SocketError::ConnectionClosed)
        } else {
            Ok(bytes as usize)
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, SocketError> {
        let bytes = unsafe { c_wut::send(self.0, buf.as_ptr() as *const _, buf.len(), 0) };

        if bytes < 0 {
            Err(SocketError::from_errno())
        } else if bytes == 0 {
            Err(SocketError::ConnectionClosed)
        } else {
            Ok(bytes as usize)
        }
    }

    pub fn shutdown(&mut self, how: Shutdown) -> Result<(), SocketError> {
        let how = match how {
            Shutdown::Read => c_wut::SHUT_RD,
            Shutdown::Write => c_wut::SHUT_WR,
            Shutdown::Both => c_wut::SHUT_RDWR,
        } as i32;

        let success = unsafe { c_wut::shutdown(self.0, how) };
        if success < 0 {
            Err(SocketError::from_errno())
        } else {
            Ok(())
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        if self.0 != -1 {
            unsafe {
                /*
                SHUTDOWN CAUSES THE CRASHES!
                */
                // let s = c_wut::shutdown(self.0, c_wut::SHUT_RDWR as i32);
                // crate::println!("shutdown: {s}");
                // crate::thread::sleep(crate::time::Duration::from_secs(2));
                c_wut::close(self.0);
            }
        }
    }
}
