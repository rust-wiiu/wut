// tcp

use crate::net::{
    socket::{Socket, SocketError},
    socket_addrs::ToSocketAddrs,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TcpError {
    #[error("socket failed")]
    SocketFailure(#[from] SocketError),
}

pub struct TcpListener {
    inner: Socket,
}

impl TcpListener {
    pub fn bind(addr: impl ToSocketAddrs) -> Result<Self, TcpError> {
        let socket = Socket::tcp()?;
        socket.bind(addr)?;

        //
        //
        //
        //
        //
        //
        //

        todo!()
    }
}

pub struct TcpStream {
    inner: Socket,
}
