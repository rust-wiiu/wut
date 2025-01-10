// tcp

use crate::net::{
    socket::{Shutdown, Socket, SocketError},
    socket_addrs::ToSocketAddrs,
};
use core::{net::SocketAddrV4, time::Duration};

#[derive(Debug)]
pub struct TcpListener {
    socket: Socket,
    pub address: SocketAddrV4,
}

impl TcpListener {
    pub fn bind(address: impl ToSocketAddrs) -> Result<Self, SocketError> {
        let socket = Socket::tcp()?;
        let address = socket.bind(address)?;
        socket.listen(16)?;

        Ok(Self { socket, address })
    }

    pub fn accept(&self) -> Result<(TcpStream, SocketAddrV4), SocketError> {
        let (client, address) = self.socket.accept()?;
        Ok((
            TcpStream {
                socket: client,
                address,
            },
            address,
        ))
    }

    pub fn incoming(&self) -> Incoming<'_> {
        Incoming { listener: &self }
    }
}

pub struct Incoming<'a> {
    listener: &'a TcpListener,
}

impl<'a> Iterator for Incoming<'a> {
    type Item = Result<TcpStream, SocketError>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.listener.accept().map(|p| p.0))
    }
}

#[derive(Debug)]
pub struct TcpStream {
    socket: Socket,
    pub address: SocketAddrV4,
}

impl TcpStream {
    pub fn connect() -> Self {
        todo!()
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, SocketError> {
        self.socket.read(buf)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, SocketError> {
        self.socket.write(buf)
    }

    pub fn shutdown(&mut self, how: Shutdown) -> Result<(), SocketError> {
        self.socket.shutdown(how)
    }

    pub fn non(&mut self) -> Result<Option<Duration>, SocketError> {
        todo!()
    }

    pub fn linger(&mut self) -> Result<Option<Duration>, SocketError> {
        todo!()
    }
    pub fn read_timeout(&self) -> Result<Option<Duration>, SocketError> {
        todo!()
    }

    pub fn write_timeout(&self) -> Result<Option<Duration>, SocketError> {
        todo!()
    }

    pub fn set_linger(&mut self, _linger: Option<Duration>) -> Result<(), SocketError> {
        todo!()
    }

    pub fn set_read_timeout(&mut self, _dur: Option<Duration>) -> Result<(), SocketError> {
        todo!()
    }

    pub fn set_write_timeout(&mut self, _dur: Option<Duration>) -> Result<(), SocketError> {
        todo!()
    }

    pub fn set_nonblocking(&mut self, _nonblocking: bool) -> Result<(), SocketError> {
        todo!()
    }
}
