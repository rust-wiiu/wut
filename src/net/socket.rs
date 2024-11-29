// socket

use crate::bindings as c_wut;
use crate::net::socket_addrs::{ToSocketAddrs, ToSocketAddrsError};
use core::fmt::Debug;
use core::net::Ipv4Addr;
use core::{ffi, net::SocketAddrV4};
use thiserror::Error;

pub struct Socket(ffi::c_int);

impl Debug for Socket {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Socket({})", self.0)
    }
}

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

    pub fn accept(&self) -> Result<Option<(Socket, SocketAddrV4)>, SocketError> {
        let mut addr = c_wut::sockaddr_in::default();
        let mut len = 16;

        let fd = unsafe {
            c_wut::accept(
                self.0,
                &mut addr as *mut _ as *mut c_wut::sockaddr,
                &mut len,
            )
        };

        crate::println!("addr: {:?}, {len}", addr.sin_addr.s_addr);
        crate::thread::sleep(core::time::Duration::from_secs(1));

        if fd < 0 {
            Err(SocketError::CannotAccept)
        } else if fd == 0 {
            Ok(None)
        } else {
            Ok(Some((
                Socket(fd),
                SocketAddrV4::new(Ipv4Addr::from_bits(addr.sin_addr.s_addr), addr.sin_port),
            )))
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe {
            c_wut::shutdown(self.0, c_wut::SHUT_RDWR as i32);
            close(self.0);
        }
    }
}

// I dont really like having this outside of bindgen but I dont want to "import" the entire <unistd.h> file just for this
extern "C" {
    fn close(sockfd: ffi::c_int) -> ffi::c_int;
}
