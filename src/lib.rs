#![feature(const_fn)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![no_std]

extern crate average;
extern crate bytecount;
extern crate core as std;
extern crate cpuio;
extern crate hashmap_core;
extern crate libm;
extern crate memadvise;
extern crate raw-cpuid;
extern crate spin;
extern crate ux;
extern crate volatile;

use spin::Mutex;

// others
#[macro_use] pub mod fs;
pub mod raw;
#[macro_use] pub mod mem;
pub mod net;
#[macro_use] pub mod vga;

#[cfg(test)]
pub mod test;

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
pub fn os() {	
	// INIT ERROR FINDING
	let mut err = ERROR {
		NO_INTERP = 0;
		NO_MEM_ERR = 0;
		NO_MEM_MAP_ERR = 0;
		PATHFINDER_DOWN_ERR = 0;
	}
	
	// thread 1
	thread!(1, FS_ADDRESS, "background", "fs");
	thread!(1, MEM_ADDRESS, "background", "mem");
	
	// thread 2
	thread!(2, INTERP_ADDRESS, "background", "watchdog");
	
	loop { }
}
