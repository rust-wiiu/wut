// tcp

use crate::bindings as c_wut;
use core::ffi;

pub struct TcpListener {
    socket: ffi::c_int,
}

impl TcpListener {
    pub fn bind
}
