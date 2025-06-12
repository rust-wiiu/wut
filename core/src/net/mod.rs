//! Networking primitives for TCP/UDP communication.
//!
//! This module provides networking functionality for the Transmission Control and User Datagram Protocols, as well as types for IP and socket addresses.

pub mod errno;
pub mod socket;
mod socket_addrs;
pub mod tcp;

pub use core::net::*;
pub use socket_addrs::{ToSocketAddrs, ToSocketAddrsError};
pub use tcp::{TcpListener, TcpStream};
