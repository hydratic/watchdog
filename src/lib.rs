#![no_std]

extern crate average;
extern crate bytecount;
extern crate core as std;
extern crate cpuio;
extern crate hashmap_core;
extern crate memadvise;
extern crate ux;
extern crate volatile;

// watchdog lang
pub mod interp;
pub mod lex;

// others
pub mod fs;
pub mod raw;

pub struct ERROR {
	
}

// TODO: VM load testing
pub fn os() {
	// TODO: PIC and CPUIO
	
	// thread 1
	thread!(1, FS_ADDRESS, "background", "fs");
	
	// thread 2
	thread!(2, INTERP_ADDRESS, "background", "watchdog");
	thread!(2, INTERP_ADDRESS, "foreground", "shell.wd");
}
