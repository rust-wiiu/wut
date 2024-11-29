// tcp

use crate::net::{
    socket::{Socket, SocketError},
    socket_addrs::ToSocketAddrs,
};
use core::net::SocketAddrV4;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TcpError {
    #[error("socket failed")]
    SocketFailure(#[from] SocketError),
}

#[derive(Debug)]
pub struct TcpListener {
    socket: Socket,
    pub address: SocketAddrV4,
}

impl TcpListener {
    pub fn bind(address: impl ToSocketAddrs) -> Result<Self, TcpError> {
        let socket = Socket::tcp()?;
        let address = socket.bind(address)?;
        socket.listen(16)?;

        Ok(Self { socket, address })
    }

    pub fn accept(&self) -> Result<Option<TcpStream>, TcpError> {
        if let Some((socket, address)) = self.socket.accept()? {
            Ok(Some(TcpStream { socket, address }))
        } else {
            Ok(None)
        }
    }

    pub fn incoming(&self) -> Incoming<'_> {
        Incoming { listener: &self }
    }
}

pub struct Incoming<'a> {
    listener: &'a TcpListener,
}

impl<'a> Iterator for Incoming<'a> {
    type Item = Result<Option<TcpStream>, TcpError>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.listener.accept())
    }
}

#[derive(Debug)]
pub struct TcpStream {
    socket: Socket,
    pub address: SocketAddrV4,
}
