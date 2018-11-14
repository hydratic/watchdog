// always_io.rs
// provides the kernel with io that never stops
// used for kernel-level keybinds

#![no_std]

extern crate cpuio;
#[macro_use] extern crate watchdog_fs as fs;
#[macro_use] extern crate watchdog_raw as raw;
extern crate ux;

use fs::header;
use fs::read;
use raw::cpu;
use raw::header;
use raw::ps2_keyboard;

pub fn init_io() -> char {
	get_header!();
	
	let mut pic: i2 = 1;
	if pic == 1 {
		if PIC_TYPE == "8529_PIC" {

		}
	}
	
	task!("io", SYS_THREAD);
	let character == get_io!();
	thread::thread__local_stroage(character, IO_THREAD_ID);
	
	if pic == 1 {
		if PIC_TYPE == "8529_PIC" {

		}
	}
}
		
pub fn keybinds(path: Path) {
	get_fs_header!();
}
