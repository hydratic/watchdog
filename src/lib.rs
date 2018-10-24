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
#[macro_use] extern crate watchdog_raw;
#[cfg(test)] extern crate watchdog_test;
extern crate volatile;
extern crate x86_64;

use x86_64::instructions::port::Port;
use spin::Mutex;

pub mod elf;
#[macro_use] pub mod fs;
#[macro_use] pub mod mem;
pub mod net;
#[macro_use] pub mod vga;
pub mod utils;
pub mod interrupts;
pub mod sshell;
pub mod oom;

static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });

static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});

// TODO: VM load testing
#[no_mangle]
pub fn os() {
    x86_64::instructions::interrupts::enable();
	ralloc::set_oom_handler(handler);
	
	// INIT SHELL
	sshell();
}
