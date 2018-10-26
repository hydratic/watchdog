#![no_std]

extern crate volatile;

use volatile::Volatile;

pub const COMMAND: Volatile<u16> = 300h;
pub const FACESTART: Volatile<u16> = 301h;
pub const FACESTOP: Volatile<u16> = 302h;
pub const BOUNDARY: Volatile<u16> = 303h;
pub const TRANSMITSTATUS: Volatile<u16> = 304h;
pub const TRANSMITPAGE: Volatile<u16> = 304h
pub const TRANSMITBYTECOUNT0: Volatile<u16> = 305h
pub const NCR: Volatile<u16> = 30h;
pub const TRANSMITBYTECOUNT1: Volatile<u16> = 306h;
pub const INTERRUPTSTATUS: Volatile<u16> = 307h;
pub const CURRENT: Volatile<u16> = 307h;
pub const REMOTESTARTADDRESS0: Volatile<u16> = 308h;
pub const CRDMA0: Volatile<u16> = 308h;
pub const REMOTESTARTADDRESS1: Volatile<u16> = 309h;
pub const CRDMA1: Volatile<u16> = 309h;
pub const REMOTEBYTECOUNT0: Volatile<u16> = 300ah;
pub const REMOTEBYTECOUNT1: Volatile<u16> = 300bh;
pub const RECEIVESTATUS: Volatile<u16> = 300ch;
pub const RECEIVECONFIGURATION: Volatile<u16> = 300ch;
pub const TRANSMT

pub fn send_packet(addr: &str) {
  
}
