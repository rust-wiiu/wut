// socket

use crate::bindings as c_wut;
use crate::net::ToSocketAddrs;
use core::ffi;
use thiserror::Error;

pub struct Socket(ffi::c_int);

#[derive(Debug, Error)]
pub enum SocketError {
    #[error("failed to create TCP socket")]
    TcpCreationError,
    #[error("failed to create UDP socket")]
    UdpCreationError,
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
                Err(SocketError::TcpCreationError)
            } else {
                Ok(Self(fd))
            }

            //
            //
            // https://doc.rust-lang.org/stable/src/std/sys_common/net.rs.html
            //
            // pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<TcpListener>
            //
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
                Err(SocketError::UdpCreationError)
            } else {
                Ok(Self(fd))
            }
        }
    }

    pub fn bind(&self, addr: impl ToSocketAddrs) -> Result<Self, SocketError> {
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
