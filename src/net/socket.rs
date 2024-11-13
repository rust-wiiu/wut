// socket

use crate::bindings as c_wut;
use crate::net::socket_addrs::{ToSocketAddrs, ToSocketAddrsError};
use core::ffi;
use thiserror::Error;

pub struct Socket(ffi::c_int);

#[derive(Debug, Error)]
pub enum SocketError {
    #[error("failed to create TCP socket")]
    TcpCreation,
    #[error("failed to create UDP socket")]
    UdpCreation,
    #[error("invalid address provided")]
    InvalidAddress(#[from] ToSocketAddrsError),
}

impl Socket {
    pub fn tcp() -> Result<Self, SocketError> {
        unsafe {
            let fd = c_wut::socket(
                c_wut::AF_UNSPEC as i32,
                c_wut::SOCK_STREAM as i32,
                c_wut::IPPROTO_TCP as i32,
            );

            crate::println!("fd: {fd}");

            crate::thread::sleep(core::time::Duration::from_secs(5));

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
                c_wut::AF_UNSPEC as i32,
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

    pub fn bind(&self, addr: impl ToSocketAddrs) -> Result<Self, SocketError> {
        for address in addr.to_socket_addrs()? {
            //
            crate::println!("{}", address);
            //
        }

        todo!()
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
