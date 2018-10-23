#![feature(const_fn)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![no_std]

extern crate average;
extern crate bytecount;
extern crate cpuio as io;
extern crate core as std;
extern crate hashmap_core;
extern crate libm;
extern crate memadvise;
extern crate raw-cpuid;
extern crate spin;
extern crate ux;
#[cfg(test)] extern crate watchdog_test;
extern crate volatile;
extern crate x86_64;

use x86_64::instructions::port::Port;
use spin::Mutex;

pub mod elf;
#[macro_use] pub mod fs;
#[macro_use] pub mod raw;
#[macro_use] pub mod mem;
pub mod net;
#[macro_use] pub mod vga;
pub mod utils;
pub mod interrupts;
pub mod sshell;

static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });

static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});

pub struct ERROR {
	NO_INTERP: i2,
	NO_MEM_ERR: i2,
	NO_MEM_MAP_ERR: i2,
	PATHFINDER_DOWN_ERR: i2,
}

// TODO: VM load testing
#[no_mangle]
pub fn os() {	
	unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
	
	// INIT ERROR FINDING
	let mut err = ERROR {
		NO_INTERP = 0;
		NO_MEM_ERR = 0;
		NO_MEM_MAP_ERR = 0;
		PATHFINDER_DOWN_ERR = 0;
	}
	
	// INIT SHELL
	sshell();
}
