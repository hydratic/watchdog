#![no_std]

extern crate average;
extern crate bytecount;
extern crate core as std;
extern crate cpuio;
extern crate hashmap_core;
extern crate libc;
extern crate libm;
extern crate memadvise;
extern crate raw-cpuid;
extern crate spin;
extern crate ux;
extern crate volatile;

// watchdog lang
pub mod interp;
pub mod lex;

// others
pub mod fs;
pub mod raw;

#[cfg(test)]
pub mod test;

pub struct ERROR {
	NO_INTERP: i2,
	NO_MEM_ERR: i2,
	NO_MEM_MAP_ERR: i2,
	PATHFINDER_DOWN_ERR: i2,
}

// TODO: VM load testing
pub fn os() {
	// TODO: PIC and CPUIO
	
	// INIT ERROR FINDING
	let mut err = ERROR {
		NO_INTERP = 0;
		NO_MEM_ERR = 0;
		NO_MEM_MAP_ERR = 0;
		PATHFINDER_DOWN_ERR = 0;
	}
	
	// thread 1
	thread!(1, FS_ADDRESS, "background", "fs");
	thread!(1, MEM_ADDRESS, "background", "slab");
	
	// thread 2
	thread!(2, INTERP_ADDRESS, "background", "watchdog");
	
	// thread 3
	thread!(2, INTERP_ADDRESS, "foreground", "shell.wd");
}
